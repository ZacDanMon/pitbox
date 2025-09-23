use crate::models::common::{Constructor, Driver};

use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

// TODO: Get gaps flag working.
#[allow(dead_code)]
pub struct DriverStandings {
    pub season: String,
    pub round: u32,
    pub entries: Vec<DriverEntry>,
}

#[serde_as]
#[derive(Deserialize)]
pub struct DriverEntry {
    #[serde(rename = "positionText")]
    pub position: String,
    #[serde_as(as = "DisplayFromStr")]
    pub points: f64,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructors")]
    pub constructors: Vec<Constructor>,
}
