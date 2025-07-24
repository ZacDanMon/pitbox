mod api;
mod cli;
mod models;
mod output;

use cli::Cli;

fn main() {
    let args = Cli::parse();

    // default: if no flag is supplied, just print both.
    let print_driver_standings =
        args.driver_standings || (!args.driver_standings && !args.constructor_standings);
    let print_constructor_standings =
        args.constructor_standings || (!args.driver_standings && !args.constructor_standings);

    if print_driver_standings {
        api::fetch_driver_standings(&args.season).map_or_else(
            |e| {
                eprintln!(
                    "Error fetching driver standings for {} season: {}",
                    args.season, e
                )
            },
            |standings| output::print_driver_standings_table(&standings),
        );
    }
    if print_constructor_standings {
        api::fetch_constructor_standings(&args.season).map_or_else(
            |e| {
                eprintln!(
                    "Error fetching constructor standings for {} season: {}",
                    args.season, e
                )
            },
            |standings| output::print_constructor_standings_table(&standings),
        );
    }
}
