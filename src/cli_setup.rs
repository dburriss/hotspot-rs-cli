extern crate clap;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use hotspot::shared_types::{
    BusFactorConfig, ContributorsConfig, HottestConfig, MetricsConfig, Verbosity,
};
use path_absolutize::Absolutize;
use std::env;
use std::path::Path;

pub enum CliCommand {
    About,
    BusFactor(BusFactorConfig),
    Contributors(ContributorsConfig),
    Metrics(MetricsConfig),
    Hottest(HottestConfig),
    //Recommend,
    Nothing,
}

const ABOUT_CMD: &str = "about";
const BUSFACTOR_CMD: &str = "busfactor";
const CONTRIBUTOR_CMD: &str = "contributors";
const METRICS_CMD: &str = "metrics";
const HOTTEST_CMD: &str = "hottest";

pub fn capture_input() -> App<'static, 'static> {
    // NOTE: Setting Arg::default_value effectively disables this option as it will ensure that some argument is always present.
    // From: https://docs.rs/clap/2.20.3/clap/enum.AppSettings.html
    let app = App::new("Hotspot")
        .settings(&[AppSettings::ArgRequiredElseHelp, AppSettings::ColoredHelp])
        .version("0.1")
        .author("Devon B. <devon@chimplab.co>")
        .about("Inspect source code for those hotspots based on source code metrics and change cadence")
        // FLAG: SET VERBOSITY
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Sets to verbose mode"),
        )
        // FLAG: SET TO SILENT
        .arg(
            Arg::with_name("silent")
                .short("s")
                .help("Sets to silent mode"),
        )
        // OPTION: CONFIG FILE
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONFIG_FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        // OPTION: OUTPUT FILE
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("REPORT_FILE")
                .help("Sets the custom output file (default is to the console)")
                .takes_value(true),
        )
        // ARG: INCLUDE GLOB
        .arg(
            Arg::with_name("include")
                .short("i")
                .long("include")
                .value_name("INCLUDE")
                .help("Glob representing explicit includes")
                .takes_value(true),
        )
        // COMMAND: ABOUT
        .subcommand(
            SubCommand::with_name(ABOUT_CMD)
                .about("Tells more about this CLI tool")
                .version("0.1")
                .author("Devon B. <devon@chimplab.co>"),
        )
        // COMMAND: BUS FACTOR
        .subcommand(
            SubCommand::with_name(BUSFACTOR_CMD)
                .about("Calculate bus factor of repository contributors")
                .version("0.1")
                .author("Devon B. <devon@chimplab.co>")
                // ARG: SOURCE CODE REPOSITORY
                .arg(
                    Arg::with_name("SOURCE")
                        .help("Sets the input path of source code to use")
                        .required(true)
                        .default_value("./")
                        .index(1),
                ),
        )
        // COMMAND: CONTRIBUTOR
        .subcommand(
            SubCommand::with_name(CONTRIBUTOR_CMD)
                .about("Gathers statistics on repository contributors")
                .version("0.1")
                .author("Devon B. <devon@chimplab.co>")
                // ARG: SOURCE CODE REPOSITORY
                .arg(
                    Arg::with_name("SOURCE")
                        .help("Sets the input path of source code to use")
                        .required(true)
                        .default_value("./")
                        .index(1),
                ),
        )
        // COMMAND: HOTTEST
        .subcommand(
            SubCommand::with_name(HOTTEST_CMD)
                .about("Lists most changed files")
                .version("0.1")
                .author("Devon B. <devon@chimplab.co>")
                // ARG: SOURCE CODE REPOSITORY
                .arg(
                    Arg::with_name("SOURCE")
                        .help("Sets the input path of source code to use")
                        .required(true)
                        .default_value("./")
                        .index(1),
                )
                .arg(
                    Arg::with_name("TOP")
                        .help("Sets the number on how many results are returned. '0' returns all.")
                        .required(false)
                        .default_value("0")
                        .long("top"),
                ),
        )
        // COMMAND: METRICS
        .subcommand(
            SubCommand::with_name(METRICS_CMD)
                .about("Gathers code metrics on repository")
                .version("0.1")
                .author("Devon B. <devon@chimplab.co>")
                // ARG: SOURCE CODE REPOSITORY
                .arg(
                    Arg::with_name("SOURCE")
                        .help("Sets the input path of source code to use")
                        .required(true)
                        .default_value("./")
                        .index(1),
                ),
        );
    app
}

fn verbosity(input: &ArgMatches) -> Verbosity {
    let occurrences = input.occurrences_of("verbosity");
    let verbosity: Verbosity = match occurrences {
        0 => Verbosity::Error,
        1 => Verbosity::Info,
        2 => Verbosity::Debug,
        3 => Verbosity::Trace,
        _ => {
            if input.is_present("silent") {
                Verbosity::Silent
            } else {
                Verbosity::Trace
            }
        }
    };
    verbosity
}

pub fn repository_path(source: Option<&str>) -> String {
    let current_dir = env::current_dir().unwrap();
    match source {
        Some(s) => {
            let base_dir = Path::new(s);
            let display_path = if base_dir.is_absolute() {
                base_dir.to_path_buf()
            } else {
                current_dir.join(base_dir)
            };
            String::from(display_path.absolutize().unwrap().to_str().unwrap())
        }
        None => String::from(current_dir.absolutize().unwrap().to_str().unwrap()),
    }
}

/// Parses the command line arguments into the correct config object for the passed in command
pub fn parse(arg_matches: ArgMatches) -> CliCommand {
    if arg_matches.subcommand_matches(ABOUT_CMD).is_some() {
        CliCommand::About
    } else if arg_matches.subcommand_matches(CONTRIBUTOR_CMD).is_some() {
        let cmd_matches = arg_matches.subcommand_matches(CONTRIBUTOR_CMD).unwrap();
        CliCommand::Contributors(ContributorsConfig {
            repository_path: repository_path(cmd_matches.value_of("SOURCE")),
            verbosity: verbosity(&arg_matches),
            output: "".to_string(),
            includes: "".to_string(),
            excludes: "".to_string(),
        })
    } else if arg_matches.subcommand_matches(BUSFACTOR_CMD).is_some() {
        let cmd_matches = arg_matches.subcommand_matches(BUSFACTOR_CMD).unwrap();
        CliCommand::BusFactor(BusFactorConfig {
            repository_path: repository_path(cmd_matches.value_of("SOURCE")),
            verbosity: verbosity(&arg_matches),
            output: "".to_string(),
            includes: "".to_string(),
            excludes: "".to_string(),
        })
    } else if arg_matches.subcommand_matches(HOTTEST_CMD).is_some() {
        let cmd_matches = arg_matches.subcommand_matches(HOTTEST_CMD).unwrap();
        CliCommand::Hottest(HottestConfig {
            repository_path: repository_path(cmd_matches.value_of("SOURCE")),
            verbosity: verbosity(&arg_matches),
            output: "".to_string(),
            includes: "".to_string(),
            excludes: "".to_string(),
            top: cmd_matches.value_of("TOP").unwrap().parse().unwrap(),
        })
    } else if arg_matches.subcommand_matches(METRICS_CMD).is_some() {
        let cmd_matches = arg_matches.subcommand_matches(METRICS_CMD).unwrap();
        CliCommand::Metrics(MetricsConfig {
            repository_path: repository_path(cmd_matches.value_of("SOURCE")),
            verbosity: verbosity(&arg_matches),
            output: "".to_string(),
            includes: "".to_string(),
            excludes: "".to_string(),
        })
    }
    // else if arg_matches.subcommand_matches(CONTRIBUTOR_CMD).is_some() {
    //     let cmd_matches = arg_matches.subcommand_matches(CONTRIBUTOR_CMD).unwrap();
    //     CliCommand::BusFactor();
    //     panic!("{} not implemented", )
    // }
    else {
        CliCommand::Nothing
    }
}
