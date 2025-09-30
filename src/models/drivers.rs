use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::models::common::Driver;

// Corresponds to MRData, the entire JSON response.
#[serde_as]
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DriverData {
    #[serde_as(as = "DisplayFromStr")]
    pub total: u32,
    #[serde(rename = "DriverTable")]
    pub driver_table: DriverTable,
}

// Corresponds to the `DriverTable` object in the JSON.
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DriverTable {
    pub season: String,
    #[serde(rename = "Drivers")]
    pub drivers: Vec<Driver>,
}
