mod api;
mod cli;
mod models;
mod output;

use api::AppResult;
use cli::Cli;

fn main() -> AppResult<()> {
    let args = Cli::parse();

    // default: if no flag is supplied, just print both.
    let print_driver_standings =
        args.driver_standings || (!args.driver_standings && !args.constructor_standings);
    let print_constructor_standings =
        args.constructor_standings || (!args.driver_standings && !args.constructor_standings);

    if print_driver_standings {
        let standings = api::fetch_driver_standings("current")?;
        output::print_driver_standings_table(&standings);
    }

    if print_constructor_standings {
        let standings = api::fetch_constructor_standings("current")?;
        output::print_constructor_standings_table(&standings);
    }

    Ok(())
}
