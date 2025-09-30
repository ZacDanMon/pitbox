use clap::Parser;

use api::AppResult;
use cli::{Cli, Command};

mod api;
mod cli;
mod commands;
mod models;
mod output;
mod stats;

fn main() -> AppResult<()> {
    let args = Cli::parse();

    match args.command {
        Command::Standings(a) => commands::run_standings(&a),
        Command::Results(a) => commands::run_race_results(&a),
        Command::Driver(a) => commands::run_driver_results(&a),
    }
}
