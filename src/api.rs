use crate::models::{
    common::ApiResponse, constructor_standings::ConstructorStandingsData,
    driver_standings::DriverStandingsData, race_results::RaceResultsData,
};

use reqwest::blocking::Client;
use std::sync::LazyLock;
use std::time::Duration;

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
    let url = format!("{}/{}/driverstandings/", BASE_URL, season);
    let response = CLIENT.get(&url).send()?;
    let json_response: ApiResponse<DriverStandingsData> = response.json()?;
    Ok(json_response)
}

pub fn fetch_constructor_standings(
    season: &str,
) -> AppResult<ApiResponse<ConstructorStandingsData>> {
    let url = format!("{}/{}/constructorstandings/", BASE_URL, season);
    let response = CLIENT.get(&url).send()?;
    let json_response: ApiResponse<ConstructorStandingsData> = response.json()?;
    Ok(json_response)
}

pub fn fetch_race_results(season: &str, round: &str) -> AppResult<ApiResponse<RaceResultsData>> {
    let url = format!("{}/{}/{}/results/", BASE_URL, season, round);
    let response = CLIENT.get(&url).send()?;
    let json_response: ApiResponse<RaceResultsData> = response.json()?;
    Ok(json_response)
}
