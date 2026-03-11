use std::{collections::HashMap, sync::LazyLock};

use comfy_table::modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS};
use comfy_table::{ContentArrangement, Table, TableComponent};
use pitbox::data::constructor_standings::ConstructorStandingsTable;
use pitbox::data::driver_standings::DriverStandingsTable;
use pitbox::data::race_results::{RaceOutcome, RaceTable};
use pitbox::stats::DriverStats;

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
pub fn print_driver_standings_table(standings: &DriverStandingsTable, show_gap: bool) {
    let mut headers = vec!["Pos", "Driver", "Constructor", "Points"];
    if show_gap {
        headers.push("Gap");
    }

    let mut table = build_table(headers);
    let leader_points = standings.standings[0].driver_standings[0].points;

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
            [] => "Unknown".into(),
            [only] => clean_constructor_name(&only.name),
            [.., last] => clean_constructor_name(&last.name),
        };

        let gap = gap_text(leader_points, e.points);

        let mut row = vec![
            e.position.as_deref().unwrap_or("-").into(),
            name,
            constructor_name,
            e.points.to_string(),
        ];

        if show_gap {
            row.push(gap);
        }

        table.add_row(row);
    }

    println!("F1 Drivers Standings 🏁");
    println!("{table}\n");
}

/// Print a pretty formatted table of F1 constructor standings to stdout.
pub fn print_constructor_standings_table(
    standings_table: &ConstructorStandingsTable,
    show_gap: bool,
) {
    let mut headers = vec!["Pos", "Constructor", "Points"];

    if show_gap {
        headers.push("Gap");
    }

    let mut table = build_table(headers);
    let leader_points = standings_table.standings[0].constructor_standings[0].points;

    for s in &standings_table.standings[0].constructor_standings {
        let constructor_name = format!(
            "{} {}",
            &clean_constructor_name(&s.constructor.name),
            &get_flag_emoji(&s.constructor.nationality)
        );

        let gap = gap_text(leader_points, s.points);

        let mut row = vec![
            s.position_text.to_owned(),
            constructor_name,
            s.points.to_string(),
        ];

        if show_gap {
            row.push(gap);
        }

        table.add_row(row);
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
        let finish_status = r.race_outcome(&r.position_text);
        let position = match finish_status {
            RaceOutcome::DidNotStart => "DNS",
            RaceOutcome::Disqualified => "DSQ",
            RaceOutcome::Retired => "RET",
            _ => &r.position_text,
        };

        let laps_down = leader_laps - r.laps.unwrap_or_default();
        let time_behind = match finish_status {
            RaceOutcome::Finished => match laps_down {
                0 => r.get_time(),
                1 => &format!("+{laps_down} lap"),
                _ => &format!("+{laps_down} laps"),
            },
            _ => position,
        };

        table.add_row(vec![
            position.into(),
            name,
            clean_constructor_name(&r.constructor.name),
            time_behind.into(),
            r.grid.unwrap_or_default().to_string(),
            r.points.to_string(),
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
        "Avg Grid",
        "Avg Finish",
        "Best Grid",
        "Best Finish",
        "Races",
        "RET",
        "Poles",
        "Wins",
        "Podiums",
        "Points",
    ]);

    let mut stats: Vec<DriverStats> = race_table
        .iter()
        .map(DriverStats::from_race_table)
        .collect();

    stats.sort_by(|x, y| y.points.total_cmp(&x.points));

    for s in &stats {
        table.add_row(vec![
            s.code.to_owned(),
            format!("{:.1}", s.avg_grid),
            format!("{:.1}", s.avg_finish),
            s.best_grid.to_string(),
            s.best_finish.to_string(),
            s.total_races.to_string(),
            s.ret.to_string(),
            s.poles.to_string(),
            s.wins.to_string(),
            s.podiums.to_string(),
            s.points.to_string(),
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

/// Get String value of the gap.
///
/// Either the gap to the leader as a negative value,
/// or "-" if gap is 0.
fn gap_text(leader_points: f64, points: f64) -> String {
    match points - leader_points {
        0.0 => "-".to_owned(),
        gap => gap.to_string(),
    }
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
