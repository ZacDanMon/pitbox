use crate::models::{
    constructor_standings::ConstructorStandings, driver_standings::DriverStandings,
};
use comfy_table::{
    ContentArrangement, Table, TableComponent,
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
};
use std::{collections::HashMap, sync::LazyLock};

static FLAGS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let toml = include_str!("../assets/nationality_flags.toml");
    toml::from_str::<HashMap<String, String>>(toml).expect("Invalid nationality_flag.toml.")
});

/// Print a pretty formatted table of F1 driver standings to stdout.
///
/// # Arguments
///
/// * 'standings' - slice of DriverStanding struct (position, driver info, points)
pub fn print_driver_standings_table(standings: DriverStandings) {
    let mut table = Table::new();

    table
        .load_preset(comfy_table::presets::UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .remove_style(TableComponent::HorizontalLines)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Pos", "Driver", "Team", "Points"]);

    for e in standings.entries {
        let flag = FLAGS
            .get(&e.driver.nationality)
            .map(String::as_str)
            .unwrap_or("");
        let name = format!("{} {} {flag}", e.driver.given_name, e.driver.family_name);

        let constructor_name: &str = match e.constructors.as_slice() {
            [] => "Unknown",
            [only] => &only.name,
            [.., last] => &last.name,
        };
        table.add_row(vec![
            &e.position,
            &name,
            constructor_name,
            &e.points.to_string(),
        ]);
    }

    println!("🏁 F1 Drivers Standings");
    println!("{table}\n");
}

/// Print a pretty formatted table of F1 constructor standings to stdout.
///
/// # Arguments
///
/// * `standings` - slice of ConstructorStanding struct (position, constructor, points)
pub fn print_constructor_standings_table(standings: ConstructorStandings) {
    let mut table = Table::new();

    table
        .load_preset(comfy_table::presets::UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .remove_style(TableComponent::HorizontalLines)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Pos", "Constructor", "Points"]);

    for e in standings.entries {
        table.add_row(vec![&e.position, &e.constructor.name, &e.points]);
    }

    println!("🏆 F1 Constructors Standings");
    println!("{table}\n");
}
