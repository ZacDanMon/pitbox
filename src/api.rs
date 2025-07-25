use crate::models::{
    constructor_standings::{ConstructorEntry, ConstructorStandings},
    driver_standings::{DriverEntry, DriverStandings},
};

use reqwest::{IntoUrl, blocking::Client};
use serde_json::Value;
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

    let round = str_as_u32(list, "round")?;

    let entries_node = list
        .get("DriverStandings")
        .ok_or_else(|| format!("Missing DriverStandings list for season {season}"))?;

    let entries: Vec<DriverEntry> = serde_json::from_value(entries_node.clone())
        .map_err(|e| format!("Failed to decode driver standings: {e}"))?;

    let standings = DriverStandings { round, entries };

    Ok(standings)
}

/// Fetches constructor standings for a given season ("current" or "2024").
pub fn fetch_constructor_standings(season: &str) -> AppResult<ConstructorStandings> {
    const LIST: &str = "/MRData/StandingsTable/StandingsLists/0";

    let url = format!("{}/{}/constructorstandings/", BASE_URL, season);
    let val: Value = fetch_json(url)?;

    let list = val
        .pointer(LIST)
        .ok_or_else(|| format!("No standings list for season {season}"))?;

    let round = str_as_u32(list, "round")?;

    let entries_node = list
        .get("ConstructorStandings")
        .ok_or_else(|| format!("Missing ConstructorStandings list for season {season}"))?;

    let entries: Vec<ConstructorEntry> = serde_json::from_value(entries_node.clone())
        .map_err(|e| format!("Failed to decode constructor standings: {e}"))?;

    let standings = ConstructorStandings { round, entries };
    Ok(standings)
}

fn str_as_u32(val: &Value, key: &str) -> AppResult<u32> {
    let s = val
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("Missing `{key}`"))?;

    Ok(s.parse::<u32>()?)
}
