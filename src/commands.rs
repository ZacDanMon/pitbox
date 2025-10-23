use pitbox::data::Driver;
use pitbox::data::race_results::RaceTable;
use pitbox::{self, AppResult};

use crate::cli::{DriverArgs, ResultsArgs, StandingsArgs};
use crate::output;

/// Process the standings subcommand.
pub fn run_standings(args: &StandingsArgs) -> AppResult<()> {
    if args.drivers {
        output::print_driver_standings_table(
            &pitbox::fetch_driver_standings(&args.season)?
                .mr_data
                .standings_table,
            args.gap,
        );
    } else if args.constructors {
        output::print_constructor_standings_table(
            &pitbox::fetch_constructor_standings(&args.season)?
                .mr_data
                .standings_table,
            args.gap,
        );
    }
    Ok(())
}

/// Process the results subcommand.
pub fn run_race_results(args: &ResultsArgs) -> AppResult<()> {
    output::print_race_results_table(
        &pitbox::fetch_race_results(&args.season, &args.round)?
            .mr_data
            .race_table,
    );
    Ok(())
}

/// Process the driver subcommand.
pub fn run_driver_results(args: &DriverArgs) -> AppResult<()> {
    // First need to get the list of drivers from this season.
    let drivers = pitbox::fetch_drivers(&args.season)?
        .mr_data
        .driver_table
        .drivers;
    let mut driver_ids: Vec<String> = Vec::new();

    // Check if each driver passed as an arg matches to a driver.
    // TODO: Remove this unwrap() call when I handle the teammate flag.
    for n in args.result_filter.name.as_ref().unwrap() {
        match get_driver_id(&drivers, n) {
            Some(id) => driver_ids.push(id),
            None => eprintln!("No driver found with name: {n}"),
        }
    }
    let mut tables: Vec<RaceTable> = Vec::new();

    for d in &driver_ids {
        tables.push(
            pitbox::fetch_driver_results(&args.season, d)?
                .mr_data
                .race_table,
        );
    }
    output::print_driver_results_table(&tables);
    Ok(())
}

/// Lookup a driver id.
///
/// Searches for a match using either the start of a last name,
/// or the 3 character driver identifier code.
fn get_driver_id(drivers: &[Driver], name: &str) -> Option<String> {
    let search_name = name.to_lowercase();

    drivers
        .iter()
        .find(|d| {
            let code_match = d
                .code
                .as_deref()
                .is_some_and(|code| code.eq_ignore_ascii_case(&search_name));
            let last_name_match = d.family_name.to_lowercase().starts_with(&search_name);

            code_match || last_name_match
        })
        .map(|d| d.id.to_string())
}
