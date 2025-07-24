use serde::Deserialize;

/// Mirrors exactly the JSON you get from `/driverstandings/`
#[derive(Deserialize)]
pub struct DriverStandingsResponse {
    #[serde(rename = "MRData")]
    pub mr_data: DriverStandingsMRData,
}

#[derive(Deserialize)]
pub struct DriverStandingsMRData {
    #[serde(rename = "StandingsTable")]
    pub standings_table: DriverStandingsTable,
}

#[derive(Deserialize)]
pub struct DriverStandingsTable {
    #[serde(rename = "StandingsLists")]
    pub standings_lists: Vec<DriverStandingsList>,
}

#[derive(Deserialize)]
pub struct DriverStandingsList {
    // pub season: String,
    // pub round: String,
    #[serde(rename = "DriverStandings")]
    pub driver_standings: Vec<DriverStanding>,
}

#[derive(Deserialize)]
pub struct DriverStanding {
    #[serde(rename = "positionText")]
    pub position: String,
    pub points: String,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructors")]
    pub constructors: Vec<Constructor>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
    pub given_name: String,
    pub family_name: String,
    pub nationality: String,
}

#[derive(Deserialize)]
pub struct Constructor {
    pub name: String,
}
