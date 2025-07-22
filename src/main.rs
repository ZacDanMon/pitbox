mod api;
mod cli;
mod models;

use api::{fetch_constructor_standings, fetch_driver_standings};
use cli::Cli;

fn main() -> api::AppResult<()> {
    let args = Cli::parse();

    // default: if no flag is supplied, just print both.
    let want_drivers = args.drivers || (!args.drivers && !args.constructors);
    let want_constructors = args.constructors || (!args.drivers && !args.constructors);

    if want_drivers {
        let standings = fetch_driver_standings("current")?;
        println!("🏁 F1 Drivers Standings\n");
        for ds in standings {
            println!(
                "{}. {} {} — {} pts ({})",
                ds.position,
                ds.driver.given_name,
                ds.driver.family_name,
                ds.points,
                ds.driver.nationality
            );
        }
        println!();
    }

    if want_constructors {
        let standings = fetch_constructor_standings("current")?;
        println!("🏆 Constructors Standings\n");
        for cs in standings {
            println!(
                "{}. {} — {} pts",
                cs.position, cs.constructor.name, cs.points,
            );
        }
    }

    Ok(())
}
