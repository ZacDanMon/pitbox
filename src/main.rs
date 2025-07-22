mod models;

use models::{ApiResponse, Constructor, ConstructorStanding, Driver, DriverStanding};
use reqwest::blocking::get;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let season = "current";
    let driver_url = format!("https://api.jolpi.ca/ergast/f1/{}/driverstandings/", season);
    let resp: ApiResponse = get(&driver_url)?.json()?;
    let list = &resp.mr_data.standings_table.standings_lists[0];

    println!(
        "🏁 F1 {} Drivers Standings (Round {})\n",
        list.season, list.round
    );
    for ds in list.driver_standings.as_ref().unwrap() {
        println!(
            "{}. {} {} — {} pts ({})",
            ds.position,
            ds.driver.given_name,
            ds.driver.family_name,
            ds.points,
            ds.driver.nationality
        );
    }

    let constructor_url = format!(
        "https://api.jolpi.ca/ergast/f1/{}/constructorstandings/",
        season
    );
    let resp: ApiResponse = get(&constructor_url)?.json()?;
    let list = &resp.mr_data.standings_table.standings_lists[0];

    println!(
        "\n🏁 F1 {} Constructors Standings (Round {})\n",
        list.season, list.round
    );
    for cs in list.constructor_standings.as_ref().unwrap() {
        println!(
            "{}. {} — {} pts",
            cs.position, cs.constructor.name, cs.points,
        );
    }

    Ok(())
}
