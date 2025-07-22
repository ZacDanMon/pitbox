use serde::Deserialize;

/// Mirrors exactly the JSON you get from `/constructorstandings/`
#[derive(Deserialize)]
pub struct ConstructorStandingsResponse {
    #[serde(rename = "MRData")]
    pub mr_data: ConstructorStandingsMRData,
}

#[derive(Deserialize)]
pub struct ConstructorStandingsMRData {
    #[serde(rename = "StandingsTable")]
    pub standings_table: ConstructorStandingsTable,
}

#[derive(Deserialize)]
pub struct ConstructorStandingsTable {
    #[serde(rename = "StandingsLists")]
    pub standings_lists: Vec<ConstructorStandingsList>,
}

#[derive(Deserialize)]
pub struct ConstructorStandingsList {
    // pub season: String,
    // pub round: String,
    #[serde(rename = "ConstructorStandings")]
    pub constructor_standings: Vec<ConstructorStanding>,
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
pub struct Constructor {
    pub name: String,
}
