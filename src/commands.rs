use crate::{api, api::AppResult, cli::StandingsArgs, output};

pub fn run_standings(args: &StandingsArgs) -> AppResult<()> {
    if args.drivers {
        output::print_driver_standings_table(&api::fetch_driver_standings(&args.season)?);
    } else if args.constructors {
        output::print_constructor_standings_table(&api::fetch_constructor_standings(&args.season)?);
    }
    Ok(())
}
