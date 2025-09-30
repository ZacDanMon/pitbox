use crate::api::{self, AppResult};
use crate::cli::{DriverArgs, ResultsArgs, StandingsArgs};
use crate::models::common::Driver;
use crate::models::race_results::RaceTable;
use crate::output;

pub fn run_standings(args: &StandingsArgs) -> AppResult<()> {
    if args.drivers {
        output::print_driver_standings_table(
            &api::fetch_driver_standings(&args.season)?
                .mr_data
                .standings_table,
        );
    } else if args.constructors {
        output::print_constructor_standings_table(
            &api::fetch_constructor_standings(&args.season)?
                .mr_data
                .standings_table,
        );
    }
    Ok(())
}

pub fn run_race_results(args: &ResultsArgs) -> AppResult<()> {
    output::print_race_results_table(
        &api::fetch_race_results(&args.season, &args.round)?
            .mr_data
            .race_table,
    );
    Ok(())
}

pub fn run_driver_results(args: &DriverArgs) -> AppResult<()> {
    let drivers = api::fetch_drivers(&args.season)?
        .mr_data
        .driver_table
        .drivers;
    let mut driver_ids: Vec<String> = Vec::new();

    for n in &args.name {
        match get_driver_id(&drivers, n) {
            Some(id) => driver_ids.push(id),
            None => eprintln!("No driver found with name: {n}"),
        }
    }
    let mut tables: Vec<RaceTable> = Vec::new();

    for d in &driver_ids {
        tables.push(
            api::fetch_driver_results(&args.season, d)?
                .mr_data
                .race_table,
        );
    }
    output::print_driver_results_table(&tables);
    Ok(())
}

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
