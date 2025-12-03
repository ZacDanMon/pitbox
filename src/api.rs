use std::error::Error;
use std::sync::LazyLock;
use std::time::Duration;

use reqwest::blocking::Client;
use serde::de::DeserializeOwned;

use crate::models::common::ApiResponse;
use crate::models::constructor_standings::ConstructorStandingsData;
use crate::models::driver_standings::DriverStandingsData;
use crate::models::drivers::DriverData;
use crate::models::race_results::RaceResultsData;

const BASE_URL: &str = "https://api.jolpi.ca/ergast/f1";

pub type AppResult<T> = Result<T, Box<dyn Error>>;

/// Global, lazily‑initialized blocking client.
/// Reuses TCP pools and applies a 10‑second request timeout.
static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client")
});

/// Generic fetch function to make an API request.
///
/// Makes a GET request to a URL and tries to deserialize into `T`.
fn fetch<T: DeserializeOwned>(url: &str) -> AppResult<T> {
    Ok(CLIENT.get(url).send()?.json::<T>()?)
}

/// Fetches driver standings for a given season ("current" or "2024").
pub fn fetch_driver_standings(season: &str) -> AppResult<ApiResponse<DriverStandingsData>> {
    fetch(&format!("{BASE_URL}/{season}/driverstandings/"))
}

/// Fetches constructor standings for a given season ("current" or "2024").
pub fn fetch_constructor_standings(
    season: &str,
) -> AppResult<ApiResponse<ConstructorStandingsData>> {
    fetch(&format!("{BASE_URL}/{season}/constructorstandings/"))
}

/// Fetches race results for a given round ("last" or "13") in a season ("current" or "2024").
pub fn fetch_race_results(season: &str, round: &str) -> AppResult<ApiResponse<RaceResultsData>> {
    fetch(&format!("{BASE_URL}/{season}/{round}/results/"))
}

/// Fetches driver results for a given season ("current" or "2024") using a driver id.
pub fn fetch_driver_results(season: &str, driver: &str) -> AppResult<ApiResponse<RaceResultsData>> {
    fetch(&format!("{BASE_URL}/{season}/drivers/{driver}/results/"))
}

/// Fetches all drivers that raced in a given season.
pub fn fetch_drivers(season: &str) -> AppResult<ApiResponse<DriverData>> {
    fetch(&format!("{BASE_URL}/{season}/drivers/"))
}

/// Fetches all drivers that raced in a given season for this constructor.
pub fn fetch_drivers_constructor(
    season: &str,
    constructor: &str,
) -> AppResult<ApiResponse<DriverData>> {
    fetch(&format!(
        "{BASE_URL}/{season}/constructors/{constructor}/drivers/"
    ))
}
