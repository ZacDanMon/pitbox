use serde::Deserialize;

pub struct ConstructorStandings {
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constructor {
    pub name: String,
}
