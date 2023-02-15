mod cli_setup;
use crate::cli_setup::CliCommand;
use crate::commands::{about_cmd, busfactor_cmd, contributors_cmd, hottest_cmd, metrics_cmd};

mod commands;

fn main() {
    let app = cli_setup::capture_input();
    let matches = app.get_matches();
    let command = cli_setup::parse(matches);

    match command {
        CliCommand::Nothing => {
            // Help will be printed by default.
            ()
        }
        CliCommand::About => {
            about_cmd::execute();
        }
        CliCommand::Contributors(config) => contributors_cmd::execute(config),
        CliCommand::BusFactor(config) => busfactor_cmd::execute(config),
        CliCommand::Metrics(config) => {
            metrics_cmd::execute(config);
        }
        CliCommand::Hottest(config) => hottest_cmd::execute(config),
        //CliCommand::Recommend => {}
    };
}
