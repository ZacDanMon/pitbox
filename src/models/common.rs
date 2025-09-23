use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constructor {
    pub name: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
    pub code: Option<String>,
    pub given_name: String,
    pub family_name: String,
    pub nationality: String,
}
