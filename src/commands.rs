use crate::{api, api::AppResult, cli::ResultsArgs, cli::StandingsArgs, output};

pub fn run_standings(args: &StandingsArgs) -> AppResult<()> {
    if args.drivers {
        output::print_driver_standings_table(api::fetch_driver_standings(&args.season)?);
    } else if args.constructors {
        output::print_constructor_standings_table(api::fetch_constructor_standings(&args.season)?);
    }
    Ok(())
}

pub fn run_race_results(args: &ResultsArgs) -> AppResult<()> {
    output::print_race_results_table(
        api::fetch_race_results(&args.season, &args.round)?
            .mr_data
            .race_table,
    );
    Ok(())
}
