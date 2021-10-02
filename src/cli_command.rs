extern crate clap;

use clap::{App, ArgMatches};
use self::clap::{AppSettings, Arg, SubCommand};

pub enum CliCommand {
    About,
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
        // COMMAND: ABOUT
        .subcommand(SubCommand::with_name("about")
            .about("Tells more about the CLI tool"))
        .version("0.1")
        .author("Devon B. <devon@chimplab.co>");
    app
}

pub fn parse(arg_matches: ArgMatches) -> CliCommand {
    if arg_matches.subcommand_matches("about").is_some() {
        CliCommand::About
    }
    else {
        CliCommand::Nothing
    }
}