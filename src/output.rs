use crate::models::{constructor_standings::ConstructorStanding, driver_standings::DriverStanding};
use comfy_table::{
    ContentArrangement, Table, TableComponent,
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
};

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
        let name = format!("{} {}", s.driver.given_name, s.driver.family_name);
        table.add_row(vec![&s.position, &name, &s.points]);
    }

    println!("🏁 F1 Drivers Standings");
    println!("{table}\n");
}

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
