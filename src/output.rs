use crate::models::{constructor_standings::ConstructorStanding, driver_standings::DriverStanding};
use comfy_table::{
    ContentArrangement, Table, TableComponent,
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
};

/// Print a pretty formatted table of F1 driver standings to stdout.
///
/// # Arguments
///
/// * 'standings' - slice of DriverStanding struct (position, driver info, points)
pub fn print_driver_standings_table(standings: &[DriverStanding]) {
    let mut table = Table::new();

    table
        .load_preset(comfy_table::presets::UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .remove_style(TableComponent::HorizontalLines)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Pos", "Driver", "Points"]);

    for s in standings {
        let flag = flag_for(&s.driver.nationality);
        let name = format!("{} {} {flag}", s.driver.given_name, s.driver.family_name);
        table.add_row(vec![&s.position, &name, &s.points]);
    }

    println!("🏁 F1 Drivers Standings");
    println!("{table}\n");
}

/// Print a pretty formatted table of F1 constructor standings to stdout.
///
/// # Arguments
///
/// * `standings` - slice of ConstructorStanding struct (position, constructor, points)
pub fn print_constructor_standings_table(standings: &[ConstructorStanding]) {
    let mut table = Table::new();

    table
        .load_preset(comfy_table::presets::UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .remove_style(TableComponent::HorizontalLines)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Pos", "Constructor", "Points"]);

    for s in standings {
        table.add_row(vec![&s.position, &s.constructor.name, &s.points]);
    }

    println!("🏆 F1 Constructors Standings");
    println!("{table}\n");
}

/// Return the emoji flag for a given nationality string.
///
/// If the nationality is unrecognized, returns an empty string.
///
/// # Examples
///
/// ```rust
/// assert_eq!(flag_for("Dutch"), "🇳🇱");
/// assert_eq!(flag_for("Martian"), "");
/// ```
fn flag_for(nat: &str) -> &'static str {
    match nat {
        "Argentine" => "🇦🇷",
        "Australian" => "🇦🇺",
        "Brazilian" => "🇧🇷",
        "British" => "🇬🇧",
        "Canadian" => "🇨🇦",
        "Dutch" => "🇳🇱",
        "French" => "🇫🇷",
        "German" => "🇩🇪",
        "Italian" => "🇮🇹",
        "Japanese" => "🇯🇵",
        "Monegasque" => "🇲🇨",
        "New Zealander" => "🇳🇿",
        "Spanish" => "🇪🇸",
        "Thai" => "🇹🇭",
        _ => "",
    }
}
