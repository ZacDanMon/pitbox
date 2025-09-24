use crate::models::{
    constructor_standings::ConstructorStandings, driver_standings::DriverStandings,
    race_results::RaceTable,
};
use comfy_table::{
    ContentArrangement, Table, TableComponent,
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
};
use std::{collections::HashMap, sync::LazyLock};

// Remove this substring from constructor names.
const REMOVE_STR: &str = "F1 Team";

static FLAGS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let toml = include_str!("../assets/nationality_flags.toml");
    toml::from_str::<HashMap<String, String>>(toml).expect("Invalid nationality_flag.toml.")
});

/// Print a pretty formatted table of F1 driver standings to stdout.
pub fn print_driver_standings_table(standings: DriverStandings) {
    let mut table = build_table(vec!["Pos", "Driver", "Constructor", "Points"]);

    for e in standings.entries {
        let name = name_with_country_flag(
            &e.driver.given_name,
            &e.driver.family_name,
            &e.driver.nationality,
        );

        // For cases were a driver raced for multiple constructors in a season,
        // only display the last one.
        let constructor_name = match e.constructors.as_slice() {
            [] => "Unknown".to_string(),
            [only] => clean_constructor_name(&only.name),
            [.., last] => clean_constructor_name(&last.name),
        };

        table.add_row(vec![
            &e.position,
            &name,
            &constructor_name,
            &e.points.to_string(),
        ]);
    }

    println!("F1 Drivers Standings 🏁");
    println!("{table}\n");
}

/// Print a pretty formatted table of F1 constructor standings to stdout.
pub fn print_constructor_standings_table(standings: ConstructorStandings) {
    let mut table = build_table(vec!["Pos", "Constructor", "Points"]);

    for e in standings.entries {
        let constructor_name = clean_constructor_name(&e.constructor.name);
        table.add_row(vec![&e.position, &constructor_name, &e.points]);
    }

    println!("F1 Constructors Standings 🏆");
    println!("{table}\n");
}

/// Print a pretty formatted table of F1 race results to stdout.
pub fn print_race_results_table(race_table: RaceTable) {
    let mut table = build_table(vec![
        "Pos",
        "Driver",
        "Constructor",
        "Time",
        "Grid",
        "Points",
    ]);

    let leader_laps = race_table.races[0].results[0].laps.unwrap_or_default();

    for r in &race_table.races[0].results {
        let name = name_with_country_flag(
            &r.driver.given_name,
            &r.driver.family_name,
            &r.driver.nationality,
        );
        let position = match r.position_text.as_str() {
            "R" => "RET",
            _ => r.position_text.as_str(),
        };

        let laps_down = leader_laps - r.laps.unwrap_or_default();

        let time_behind = match (laps_down, position) {
            (_, "RET") => "DNF".to_string(),
            (0, _) => r.get_time().to_string(),
            (1, _) => format!("+{laps_down} lap"),
            (_, _) => format!("+{laps_down} laps"),
        };

        table.add_row(vec![
            position,
            &name,
            &clean_constructor_name(&r.constructor.name),
            &time_behind,
            &r.grid.as_deref().unwrap_or_default(),
            &r.points.to_string(),
        ]);
    }

    println!(
        "{} {} Results 🏁",
        race_table.races[0].season, race_table.races[0].race_name
    );
    println!("{table}\n");
}

/// Removes substring "F1" from a constructor name.
fn clean_constructor_name(name: &str) -> String {
    let clean_name = name.replace(REMOVE_STR, "");
    clean_name
}

/// Builds a formatted string with Driver first name, last name, and country flag.
fn name_with_country_flag(first_name: &str, last_name: &str, nationality: &str) -> String {
    format!(
        "{first_name} {last_name} {}",
        FLAGS.get(nationality).map(String::as_str).unwrap_or("")
    )
}

/// Build a comfy_table for output with the provided headers.
fn build_table(headers: Vec<&str>) -> Table {
    let mut table = Table::new();

    table
        .load_preset(comfy_table::presets::UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .remove_style(TableComponent::HorizontalLines)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(headers);

    table
}
