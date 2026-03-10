use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::models::common::{Constructor, Driver};

// Corresponds to MRData, the entire JSON response.
#[derive(Deserialize)]
pub struct DriverStandingsData {
    #[serde(rename = "StandingsTable")]
    pub standings_table: DriverStandingsTable,
}

// Corresponds to the `StandingsTable` object in the JSON.
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DriverStandingsTable {
    pub season: String,
    pub round: String,
    #[serde(rename = "StandingsLists")]
    pub standings: Vec<DriverStandingsList>,
}

#[derive(Deserialize)]
#[serde_as]
#[allow(dead_code)]
pub struct DriverStandingsList {
    pub season: String,
    pub round: String,
    #[serde(rename = "DriverStandings")]
    pub driver_standings: Vec<DriverStandingsEntry>,
}

#[serde_as]
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DriverStandingsEntry {
    pub position: Option<String>,
    #[serde(rename = "positionText")]
    pub position_text: String,
    #[serde_as(as = "DisplayFromStr")]
    pub points: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub wins: u32,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructors")]
    pub constructors: Vec<Constructor>,
}
