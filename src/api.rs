use crate::models::{
    constructor_standings::ConstructorStandingsResponse,
    driver_standings::{DriverEntry, DriverStandings},
    race_results::ApiResponse,
};

use reqwest::{IntoUrl, blocking::Client};
use serde::Deserialize;
use serde_json::Value;
use serde_with::{DisplayFromStr, serde_as};
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

#[serde_as]
#[derive(Deserialize)]
struct StandingsData {
    season: String,
    #[serde_as(as = "DisplayFromStr")]
    round: u32,
}

fn fetch_json(url: impl IntoUrl) -> AppResult<Value> {
    Ok(CLIENT.get(url).send()?.json()?)
}

/// Fetches driver standings for a given season ("current" or "2024").
pub fn fetch_driver_standings(season: &str) -> AppResult<DriverStandings> {
    const LIST: &str = "/MRData/StandingsTable/StandingsLists/0";

    let url = format!("{}/{}/driverstandings/", BASE_URL, season);
    let val: Value = fetch_json(url)?;

    let list = val
        .pointer(LIST)
        .ok_or_else(|| format!("No standings list for season {season}"))?;

    let standings_data: StandingsData = serde_json::from_value(list.clone())
        .map_err(|e| format!("Failed to decode standings data: {e}"))?;

    let entries_node = list
        .get("DriverStandings")
        .ok_or_else(|| format!("Missing DriverStandings list for season {season}"))?;

    let entries: Vec<DriverEntry> = serde_json::from_value(entries_node.clone())
        .map_err(|e| format!("Failed to decode driver standings: {e}"))?;

    let standings = DriverStandings {
        season: standings_data.season,
        round: standings_data.round,
        entries,
    };

    Ok(standings)
}

pub fn fetch_constructor_standings(season: &str) -> AppResult<ConstructorStandingsResponse> {
    let url = format!("{}/{}/constructorstandings/", BASE_URL, season);
    let response = CLIENT.get(&url).send()?;
    let json_response: ConstructorStandingsResponse = response.json()?;
    Ok(json_response)
}

pub fn fetch_race_results(season: &str, round: &str) -> AppResult<ApiResponse> {
    let url = format!("{}/{}/{}/results/", BASE_URL, season, round);
    let response = CLIENT.get(&url).send()?;
    let json_response: ApiResponse = response.json()?;
    Ok(json_response)
}
