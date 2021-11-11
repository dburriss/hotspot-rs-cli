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
        //CliCommand::BusFactor => {}
        CliCommand::Metrics(config) => {
            metrics_cmd::execute(config);
        }
        //CliCommand::Recommend => {}
    };
}
