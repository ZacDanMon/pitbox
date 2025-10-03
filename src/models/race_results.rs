use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::models::common::{Constructor, Driver};

/// A driver's outcome from the `RaceResult`.
pub enum RaceOutcome {
    DidNotStart,
    Disqualified,
    Finished,
    Retired,
    Unknown,
}

// Corresponds to MRData, the entire JSON response.
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
    #[serde(rename = "raceName")]
    pub name: String,
    #[serde(rename = "Circuit")]
    pub circuit: Circuit,
    pub date: String,
    pub time: Option<String>,
    #[serde(rename = "Results")]
    pub results: Vec<RaceResult>,
}

// Corresponds to the `Circuit` object.
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Circuit {
    #[serde(rename = "circuitId")]
    pub id: String,
    #[serde(rename = "circuitName")]
    pub name: String,
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
    #[serde_as(as = "DisplayFromStr")]
    pub position: u32,
    #[serde(rename = "positionText")]
    pub position_text: String,
    #[serde_as(as = "DisplayFromStr")]
    pub points: f64,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub grid: Option<u32>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub laps: Option<u32>,
    pub status: Option<String>,
    #[serde(rename = "Time")]
    pub time: Option<RaceResultTime>,
    #[serde(rename = "FastestLap")]
    pub fastest_lap: Option<FastestLap>,
}

impl RaceResult {
    pub fn get_time(&self) -> &str {
        match &self.time {
            Some(race_result_time) => &race_result_time.time,
            None => "",
        }
    }

    /// Convert `position_text` to a `RaceOutcome`.
    pub fn race_outcome(&self, position_text: &str) -> RaceOutcome {
        // If the string is a number, the driver finished the race.
        if position_text.parse::<i32>().is_ok() {
            RaceOutcome::Finished
        } else {
            match position_text {
                "W" => RaceOutcome::DidNotStart,
                "D" => RaceOutcome::Disqualified,
                "R" => RaceOutcome::Retired,
                _ => {
                    eprintln!("Unknown position_text: {position_text}");
                    RaceOutcome::Unknown
                }
            }
        }
    }
}

// Corresponds to the `Time` object under RaceResult.
#[serde_as]
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct RaceResultTime {
    #[serde_as(as = "DisplayFromStr")]
    pub millis: u32,
    pub time: String,
}

// Corresponds to the `FastestLap` object.
#[serde_as]
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FastestLap {
    #[serde_as(as = "DisplayFromStr")]
    pub rank: u32,
    pub lap: String,
    #[serde(rename = "Time")]
    pub time: FastestLapTime,
}

// Corresponds to the `Time` object under FastestLap.
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FastestLapTime {
    pub time: String,
}
