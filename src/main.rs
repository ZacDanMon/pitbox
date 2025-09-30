use clap::Parser;
use pitbox::api::AppResult;

use cli::{Cli, Command};

mod cli;
mod commands;
mod output;

fn main() -> AppResult<()> {
    let args = Cli::parse();

    match args.command {
        Command::Standings(a) => commands::run_standings(&a),
        Command::Results(a) => commands::run_race_results(&a),
        Command::Driver(a) => commands::run_driver_results(&a),
    }
}
