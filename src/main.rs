mod shared_types;
mod cli_setup;
mod about_cmd;
mod repository;
mod contributors;
mod metrics_cmd;
mod contributors_cmd;

use crate::cli_setup::CliCommand;

fn main() {
    let app = cli_setup::capture_input();
    let matches = app.get_matches();
    let command = cli_setup::parse(matches);

    match command  {
        CliCommand::Nothing => {
            // Help will be printed by default.
            ()
        }
        CliCommand::About => {
            about_cmd::execute();
        },
        CliCommand::Contributors(config) => {
            contributors_cmd::execute(config)
        }
        CliCommand::BusFactor => {}
        CliCommand::Metrics(config) => {
            metrics_cmd::execute(config);
        }
        CliCommand::Recommend => {}
    };

//     // Gets a value for config if supplied by user, or defaults to "default.conf"
//     //let config = matches.value_of("config");
//     //println!("Value for config: {}", config);
//
//     // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
//     // required we could have used an 'if let' to conditionally get the value)
//     println!("Repository path: {}", input.value_of("SOURCE").unwrap());
//
//     // Vary the output based on how many times the user used the "verbose" flag
//     // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
//     let verbosity : Verbosity =
//         match input.occurrences_of("v") {
//             1 => Verbosity::Info,
//             2 => Verbosity::Debug,
//             3 => Verbosity::Trace,
//             _ => if input.is_present("silent") { Verbosity::Silent } else { Verbosity::Info }
//         };
//     println!("Verbosity: {}", verbosity.to_string());
//
//     // You can handle information about subcommands by requesting their matches by name
//     // (as below), requesting just the name used, or both at the same time
//     if input.subcommand_matches("about").is_some() {
//         println!(
// "The 'hotspot' CLI contains various tools for inspecting source code to try find those areas that warrant a closer look.
// It uses code metrics and combines that with source control history to highlight issues not just due to metrics but those metrics in light of how often and how frequent code changes.
// It also considers contributor diversity and warns on bus factor for complex and/or high change code.
// Supports: C, C++, C#, Go, Java, Python, Rust");
//     }
//
//     // if let Some(matches) = matches.subcommand_matches("contributors") {
//     //     let repoPath = matches.value_of("SOURCE");
//     // }
}
