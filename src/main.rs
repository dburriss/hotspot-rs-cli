extern crate clap;
use clap::{Arg, App, SubCommand};
use std::fmt::{self, Debug};

#[derive(Debug)]
enum Verbosity {
    Silent,
    Info,
    Debug,
    Trace
}

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(dead_code)]
// TODO: A context object that is told about each step starting, ending, progress, etc.
struct BaseConfig {
    operation: String,
    repository_path: String,
    verbosity: Verbosity,
    output: String,
    includes: String,
    excludes: String
}

fn main() {
    let matches = App::new("Hotspot")
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
                        // ARG: OUTPUT FILE
                        .arg(Arg::with_name("SOURCE")
                            .help("Sets the input path of source code to use")
                            .required(true)
                            .default_value("./")
                            .index(1))
                        // COMMAND: CONTRIBUTOR
                        .subcommand(SubCommand::with_name("contributors")
                            .about("Gathers statistics on repository contributors")
                            .version("0.1")
                            .author("Devon B. <devon@chimplab.co>")
                            .arg(Arg::with_name("debug")
                                .short("d")
                                .help("Print debug information verbosely")))
                        // COMMAND: ABOUT
                        .subcommand(SubCommand::with_name("about")
                            .about("Tells more about the CLI tool"))
                            .version("0.1")
                            .author("Devon B. <devon@chimplab.co>")
                        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    //let config = matches.value_of("config");
    //println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Repository path: {}", matches.value_of("SOURCE").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    let verbosity : Verbosity = 
        match matches.occurrences_of("v") {
            1 => Verbosity::Info,
            2 => Verbosity::Debug,
            3 => Verbosity::Trace,
            _ => if matches.is_present("silent") { Verbosity::Silent } else { Verbosity::Info }
        };
    println!("Verbosity: {}", verbosity.to_string());
    
    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if matches.subcommand_matches("about").is_some() {
        println!(
"The 'hotspot' CLI contains various tools for inspecting source code to try find those areas that warrant a closer look. 
It uses code metrics and combines that with source control history to highlight issues not just due to metrics but those metrics in light of how often and how frequent code changes. 
It also considers contributor diversity and warns on bus factor for complex and/or high change code.
Supports: C, C++, C#, Go, Java, Python, Rust");
    }

    // if let Some(matches) = matches.subcommand_matches("contributors") {
    //     let repoPath = matches.value_of("SOURCE");
    // }
}
