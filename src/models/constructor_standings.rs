use crate::models::common::Constructor;

use serde::Deserialize;

// TODO: Get gaps flag working.
#[allow(dead_code)]
pub struct ConstructorStandings {
    pub season: String,
    pub round: u32,
    pub entries: Vec<ConstructorEntry>,
}

#[derive(Deserialize)]
pub struct ConstructorEntry {
    pub position: String,
    pub points: String,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
}
