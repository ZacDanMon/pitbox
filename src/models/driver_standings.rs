use serde::Deserialize;

pub struct DriverStandings {
    pub round: u32,
    pub entries: Vec<DriverEntry>,
}

#[derive(Deserialize)]
pub struct DriverEntry {
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
