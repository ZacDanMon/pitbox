use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "MRData")]
    pub mr_data: MRData,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MRData {
    pub standings_table: StandingsTable,
}

#[derive(Deserialize)]
pub struct StandingsTable {
    // season: String,
    // round: String,
    #[serde(rename = "StandingsLists")]
    pub standings_lists: Vec<StandingsList>,
}

#[derive(Deserialize)]
pub struct StandingsList {
    pub season: String,
    pub round: String,

    #[serde(rename = "DriverStandings")]
    pub driver_standings: Option<Vec<DriverStanding>>,
    #[serde(rename = "ConstructorStandings")]
    pub constructor_standings: Option<Vec<ConstructorStanding>>,
}

#[derive(Deserialize)]
pub struct DriverStanding {
    pub position: String,
    pub points: String,

    #[serde(rename = "Driver")]
    pub driver: Driver,
    // Constructors: Vec<Constructor>,
}

#[derive(Deserialize)]
pub struct ConstructorStanding {
    pub position: String,
    pub points: String,

    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
    pub given_name: String,
    pub family_name: String,
    pub nationality: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constructor {
    pub name: String,
    // nationality: String,
}
