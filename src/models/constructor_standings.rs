use crate::models::common::Constructor;

use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

// Corresponds to MRData, the entire JSON response.
#[derive(Deserialize)]
pub struct ConstructorStandingsData {
    #[serde(rename = "StandingsTable")]
    pub standings_table: ConstructorStandingsTable,
}

// Corresponds to the `RaceTable` object in the JSON.
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ConstructorStandingsTable {
    pub season: String,
    pub round: String,
    #[serde(rename = "StandingsLists")]
    pub standings: Vec<ConstructorStandingsList>,
}

#[derive(Deserialize)]
#[serde_as]
#[allow(dead_code)]
pub struct ConstructorStandingsList {
    pub season: String,
    #[serde(rename = "ConstructorStandings")]
    pub constructor_standings: Vec<ConstructorStandingsEntry>,
}

// Corresponds to a single object in the `ConstructorStandings` array.
#[serde_as]
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ConstructorStandingsEntry {
    pub position: String,
    #[serde(rename = "positionText")]
    pub position_text: String,
    #[serde_as(as = "DisplayFromStr")]
    pub points: f64,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
}
