use crate::models::common::{Constructor, Driver};

use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

// The true top-level struct that matches the entire JSON response.
#[derive(Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "MRData")]
    pub mr_data: RaceResultsData,
}

// The top-level struct that matches the overall JSON response.
#[derive(Deserialize)]
pub struct RaceResultsData {
    #[serde(rename = "RaceTable")]
    pub race_table: RaceTable,
}

// Corresponds to the `RaceTable` object in the JSON.
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct RaceTable {
    pub season: String,
    pub round: Option<String>,
    #[serde(rename = "Races")]
    pub races: Vec<Race>,
}

// Corresponds to a single `Race` object.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Race {
    pub season: String,
    pub round: String,
    pub race_name: String,
    #[serde(rename = "Circuit")]
    pub circuit: Circuit,
    pub date: String,
    pub time: String,
    #[serde(rename = "Results")]
    pub results: Vec<RaceResult>,
}

// Corresponds to the `Circuit` object.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Circuit {
    pub circuit_id: String,
    pub circuit_name: String,
    #[serde(rename = "Location")]
    pub location: Location,
}

// Corresponds to the `Location` object.
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Location {
    pub locality: String,
    pub country: String,
}

// Corresponds to a single `RaceResult` object.
#[serde_as]
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct RaceResult {
    #[serde(rename = "positionText")]
    pub position: String,
    #[serde_as(as = "DisplayFromStr")]
    pub points: f64,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    pub grid: Option<String>,
    pub laps: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "Time")]
    pub time: Option<RaceResultTime>,
    #[serde(rename = "FastestLap")]
    pub fastest_lap: Option<FastestLap>,
}

// Corresponds to the `Time` object.
#[serde_as]
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct RaceResultTime {
    #[serde_as(as = "DisplayFromStr")]
    pub millis: u32,
    pub time: String,
}

// Corresponds to the `Time` object.
#[serde_as]
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FastestLap {
    #[serde_as(as = "DisplayFromStr")]
    pub rank: u8,
    pub lap: String,
    #[serde(rename = "Time")]
    pub time: FastestLapTime,
}
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FastestLapTime {
    pub time: String,
}
