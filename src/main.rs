mod api;
mod cli;
mod models;
mod output;

use api::AppResult;
use cli::Cli;

fn main() -> AppResult<()> {
    let args = Cli::parse();

    // default: if no flag is supplied, just print both.
    let print_driver_standings = args.drivers || (!args.drivers && !args.constructors);
    let print_constructor_standings = args.constructors || (!args.drivers && !args.constructors);

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
