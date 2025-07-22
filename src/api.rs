use crate::models::{
    constructor_standings::{ConstructorStanding, ConstructorStandingsResponse},
    driver_standings::{DriverStanding, DriverStandingsResponse},
};

use reqwest::{IntoUrl, blocking::Client};
use serde::de::DeserializeOwned;
use std::sync::LazyLock;
use std::time::Duration;

const BASE_URL: &str = "https://api.jolpi.ca/ergast/f1";

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Global, lazily‑initialized blocking client.
/// Reuses TCP pools and applies a 10‑second request timeout.
static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build reqwest client")
});

/// Generic helper to GET a URL and deserialize JSON body into T.
fn fetch_json<T>(url: impl IntoUrl) -> AppResult<T>
where
    T: DeserializeOwned,
{
    Ok(CLIENT.get(url).send()?.json()?)
}

/// Fetches driver standings for a given season ("current" or "2024").
pub fn fetch_driver_standings(season: &str) -> AppResult<Vec<DriverStanding>> {
    let url = format!("{}/{}/driverstandings/", BASE_URL, season);
    let resp: DriverStandingsResponse = fetch_json(url)?;

    // We know there's at least one StandingsList, so `.into_iter().next().unwrap()` is okay here.
    let list = resp
        .mr_data
        .standings_table
        .standings_lists
        .into_iter()
        .next()
        .ok_or_else(|| format!("no driver standings list for season: {season}"))?;
    Ok(list.driver_standings)
}

/// Fetches constructor standings for a given season ("current" or "2024").
pub fn fetch_constructor_standings(season: &str) -> AppResult<Vec<ConstructorStanding>> {
    let url = format!("{}/{}/constructorstandings/", BASE_URL, season);
    let resp: ConstructorStandingsResponse = fetch_json(url)?;
    let list = resp
        .mr_data
        .standings_table
        .standings_lists
        .into_iter()
        .next()
        .ok_or_else(|| format!("no constructor standings list for season: {season}"))?;
    Ok(list.constructor_standings)
}
