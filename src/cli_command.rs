extern crate clap;

use std::env;
use std::path::{Path, PathBuf};
use clap::{App, ArgMatches};
use path_absolutize::Absolutize;
use self::clap::{AppSettings, Arg, SubCommand};
use crate::shared_types::{MetricsConfig, Verbosity};

pub enum CliCommand {
    About,
    Contributors,
    BasFactor,
    Metrics(MetricsConfig),
    Recommend,
    Nothing
}

pub fn capture_input() -> App<'static, 'static> {
    // NOTE: Setting Arg::default_value effectively disables this option as it will ensure that some argument is always present.
    // From: https://docs.rs/clap/2.20.3/clap/enum.AppSettings.html
    let app = App::new("Hotspot")
        .settings(&[AppSettings::ArgRequiredElseHelp, AppSettings::ColoredHelp])
        .version("0.1")
        .author("Devon B. <devon@chimplab.co>")
        .about("Inspect source code for those hotspots based on source code change cadence")
        // FLAG: OUTPUT FILE
        .arg(Arg::with_name("verbosity")
            .short("v")
            .multiple(true)
            .help("Sets to verbose mode"))
        // FLAG: OUTPUT FILE
        .arg(Arg::with_name("silent")
            .short("s")
            .help("Sets to silent mode"))
        // OPTION: CONFIG FILE
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("CONFIG_FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        // OPTION: OUTPUT FILE
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("REPORT_FILE")
            .help("Sets the custom output file (default is to the console)")
            .takes_value(true))
        // ARG: INCLUDE GLOB
        .arg(Arg::with_name("include")
            .short("i")
            .long("include")
            .value_name("INCLUDE")
            .help("Glob representing explicit includes")
            .takes_value(true))
        // COMMAND: CONTRIBUTOR
        .subcommand(SubCommand::with_name("contributors")
            .about("Gathers statistics on repository contributors")
            .version("0.1")
            .author("Devon B. <devon@chimplab.co>")
            // ARG: SOURCE CODE REPOSITORY
            .arg(Arg::with_name("SOURCE")
                .help("Sets the input path of source code to use")
                .required(true)
                .default_value("./")
                .index(1)))
        // COMMAND: CONTRIBUTOR
        .subcommand(SubCommand::with_name("metrics")
            .about("Gathers code metrics on repository")
            .version("0.1")
            .author("Devon B. <devon@chimplab.co>")
            // ARG: SOURCE CODE REPOSITORY
            .arg(Arg::with_name("SOURCE")
                .help("Sets the input path of source code to use")
                .required(true)
                .default_value("./")
                .index(1)))
        // COMMAND: ABOUT
        .subcommand(SubCommand::with_name("about")
            .about("Tells more about the CLI tool"))
        .version("0.1")
        .author("Devon B. <devon@chimplab.co>");
    app
}

fn verbosity(input: ArgMatches) -> Verbosity {
    let verbosity : Verbosity =
        match input.occurrences_of("v") {
            0 => Verbosity::Error,
            1 => Verbosity::Info,
            2 => Verbosity::Debug,
            3 => Verbosity::Trace,
            _ => if input.is_present("silent") { Verbosity::Silent } else { Verbosity::Trace }
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
        },
        None => {
            String::from(current_dir.absolutize().unwrap().to_str().unwrap())
        }
    }

}

pub fn parse(arg_matches: ArgMatches) -> CliCommand {
    if arg_matches.subcommand_matches("about").is_some() {
        CliCommand::About
    }
    else if arg_matches.subcommand_matches("metrics").is_some() {
        let cmd_matches = arg_matches.subcommand_matches("metrics").unwrap();
        CliCommand::Metrics(
            MetricsConfig{
                repository_path: repository_path(cmd_matches.value_of("SOURCE")),
                verbosity: verbosity(arg_matches),
                output: "".to_string(),
                includes: "".to_string(),
                excludes: "".to_string(),
            }
        )
    }
    else {
        CliCommand::Nothing
    }
}