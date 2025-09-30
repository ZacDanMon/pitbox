use std::sync::LazyLock;
use std::time::Duration;

use reqwest::blocking::Client;

use crate::models::common::ApiResponse;
use crate::models::constructor_standings::ConstructorStandingsData;
use crate::models::driver_standings::DriverStandingsData;
use crate::models::drivers::DriverData;
use crate::models::race_results::RaceResultsData;

const BASE_URL: &str = "https://api.jolpi.ca/ergast/f1";

pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Global, lazily‑initialized blocking client.
/// Reuses TCP pools and applies a 10‑second request timeout.
static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client")
});

/// Fetches driver standings for a given season ("current" or "2024").
pub fn fetch_driver_standings(season: &str) -> AppResult<ApiResponse<DriverStandingsData>> {
    let url = format!("{BASE_URL}/{season}/driverstandings/");
    let response = CLIENT.get(&url).send()?;
    let json_response: ApiResponse<DriverStandingsData> = response.json()?;
    Ok(json_response)
}

/// Fetches constructor standings for a given season ("current" or "2024").
pub fn fetch_constructor_standings(
    season: &str,
) -> AppResult<ApiResponse<ConstructorStandingsData>> {
    let url = format!("{BASE_URL}/{season}/constructorstandings/");
    let response = CLIENT.get(&url).send()?;
    let json_response: ApiResponse<ConstructorStandingsData> = response.json()?;
    Ok(json_response)
}

/// Fetches race results for a given round ("last" or "13") in a season ("current" or "2024").
pub fn fetch_race_results(season: &str, round: &str) -> AppResult<ApiResponse<RaceResultsData>> {
    let url = format!("{BASE_URL}/{season}/{round}/results/");
    let response = CLIENT.get(&url).send()?;
    let json_response: ApiResponse<RaceResultsData> = response.json()?;
    Ok(json_response)
}

/// Fetches driver results for a given season ("current" or "2024") using a driver id.
pub fn fetch_driver_results(season: &str, driver: &str) -> AppResult<ApiResponse<RaceResultsData>> {
    let url = format!("{BASE_URL}/{season}/drivers/{driver}/results/");
    let response = CLIENT.get(&url).send()?;
    let json_response: ApiResponse<RaceResultsData> = response.json()?;
    Ok(json_response)
}

/// Fetches all drivers that raced in a given season.
pub fn fetch_drivers(season: &str) -> AppResult<ApiResponse<DriverData>> {
    let url = format!("{BASE_URL}/{season}/drivers/");
    let response = CLIENT.get(&url).send()?;
    let json_response: ApiResponse<DriverData> = response.json()?;
    Ok(json_response)
}
