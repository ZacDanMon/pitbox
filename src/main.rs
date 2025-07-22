mod api;
mod models;

use crate::models::constructor_standings::ConstructorStanding;
use crate::models::driver_standings::DriverStanding;
use api::{fetch_constructor_standings, fetch_driver_standings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let season = "current";
    let drivers: Vec<DriverStanding> = fetch_driver_standings(season)?;

    println!("🏁 F1 {} Drivers Standings\n", season);
    for ds in drivers {
        let d = &ds.driver;
        println!(
            "{}. {} {} — {} pts ({})",
            ds.position, d.given_name, d.family_name, ds.points, d.nationality
        );
    }

    let constructors: Vec<ConstructorStanding> = fetch_constructor_standings(season)?;

    println!("\n🏁 F1 {} Constructors Standings\n", season);
    for cs in constructors {
        println!(
            "{}. {} — {} pts",
            cs.position, cs.constructor.name, cs.points,
        );
    }

    Ok(())
}
