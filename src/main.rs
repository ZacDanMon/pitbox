mod api;
mod cli;
mod commands;
mod models;
mod output;

use api::AppResult;
use clap::Parser;
use cli::{Cli, Command};

fn main() -> AppResult<()> {
    let args = Cli::parse();

    match args.command {
        Command::Standings(a) => commands::run_standings(&a),
        Command::Results(a) => commands::run_race_results(&a),
    }
}
