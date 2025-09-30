use std::{collections::HashMap, sync::LazyLock};

use comfy_table::modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS};
use comfy_table::{ContentArrangement, Table, TableComponent};

use crate::models::constructor_standings::ConstructorStandingsTable;
use crate::models::driver_standings::DriverStandingsTable;
use crate::models::race_results::RaceTable;
use crate::stats::DriverStats;

/// Remove this redundant substring from constructor names.
const REMOVE_STR: &str = "F1 Team";

/// Mapping of nationalities to country names read from `nationality.toml`.
static NATIONALITIES: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let toml = include_str!("../resources/nationality.toml");
    toml::from_str::<HashMap<String, String>>(toml).expect("Invalid nationality.toml.")
});

/// Mapping of country names to emoji flags read from `flags.toml`.
static FLAGS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let toml = include_str!("../resources/flags.toml");
    toml::from_str::<HashMap<String, String>>(toml).expect("Invalid flags.toml.")
});

/// Print a pretty formatted table of F1 driver standings to stdout.
pub fn print_driver_standings_table(standings: &DriverStandingsTable) {
    let mut table = build_table(vec!["Pos", "Driver", "Constructor", "Points"]);

    for e in &standings.standings[0].driver_standings {
        let name = format!(
            "{} {} {}",
            &e.driver.given_name,
            &e.driver.family_name,
            get_flag_emoji(&e.driver.nationality)
        );

        // For cases where a driver raced for multiple constructors in a season,
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
pub fn print_constructor_standings_table(standings_table: &ConstructorStandingsTable) {
    let mut table = build_table(vec!["Pos", "Constructor", "Points"]);

    for s in &standings_table.standings[0].constructor_standings {
        let constructor_name = format!(
            "{} {}",
            &s.constructor.name,
            get_flag_emoji(&s.constructor.nationality)
        );
        table.add_row(vec![
            &s.position_text,
            &constructor_name,
            &s.points.to_string(),
        ]);
    }

    println!("F1 Constructors Standings 🏆");
    println!("{table}\n");
}

/// Print a pretty formatted table of F1 race results to stdout.
pub fn print_race_results_table(race_table: &RaceTable) {
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
        let name = format!(
            "{} {} {}",
            &r.driver.given_name,
            &r.driver.family_name,
            get_flag_emoji(&r.driver.nationality)
        );
        let position = match r.position_text.as_str() {
            "R" => "RET",
            "D" => "DSQ",
            "W" => "DNS",
            _ => r.position_text.as_str(),
        };

        let laps_down = leader_laps - r.laps.unwrap_or_default();

        let time_behind = match (laps_down, position) {
            (_, "RET") => "DNF".to_string(),
            (_, "DSQ") => "DSQ".to_string(),
            (_, "DNS") => "DNS".to_string(),
            (0, _) => r.get_time().to_string(),
            (1, _) => format!("+{laps_down} lap"),
            (_, _) => format!("+{laps_down} laps"),
        };

        table.add_row(vec![
            position,
            &name,
            &clean_constructor_name(&r.constructor.name),
            &time_behind,
            &r.grid.unwrap_or_default().to_string(),
            &r.points.to_string(),
        ]);
    }

    println!(
        "{} {} Results {}",
        race_table.races[0].season,
        race_table.races[0].name,
        get_flag_emoji(&race_table.races[0].circuit.location.country),
    );
    println!("{table}\n");
}

/// Print a pretty formatted table of driver results to stdout.
pub fn print_driver_results_table(race_table: &[RaceTable]) {
    let mut table = build_table(vec![
        "Driver",
        "Races",
        "Best Grid",
        "Best Finish",
        "Avg Grid",
        "Avg Finish",
        "Poles",
        "Wins",
        "RET",
        "Points",
    ]);

    for t in race_table {
        let stats = DriverStats::from_race_table(t);

        table.add_row(vec![
            &stats.code,
            &stats.total_races.to_string(),
            &stats.best_grid.to_string(),
            &stats.best_finish.to_string(),
            &format!("{:.1}", stats.avg_grid),
            &format!("{:.1}", stats.avg_finish),
            &stats.poles.to_string(),
            &stats.wins.to_string(),
            &stats.ret.to_string(),
            &stats.points.to_string(),
        ]);
    }
    println!("Driver Results",);
    println!("{table}\n");
}

/// Removes substring "F1 Team" from a constructor name.
///
/// # Returns a new string with "F1 Team" removed from the name.
fn clean_constructor_name(name: &str) -> String {
    name.replace(REMOVE_STR, "")
}

/// Lookup the country flag using a nation key.
///
/// Key can be an adjective, e.g., German, or
/// a noun, e.g., Germany.
///
/// # Returns a String containing the emoji flag.
fn get_flag_emoji(key: &str) -> String {
    let final_key = NATIONALITIES.get(key).map_or(key, String::as_str);
    FLAGS
        .get(final_key)
        .map(String::as_str)
        .unwrap_or_default()
        .to_string()
}

/// Build a `comfy_table` for output with the provided headers.
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
