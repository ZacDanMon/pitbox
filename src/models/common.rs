use serde::Deserialize;

// Top level struct that matches the entire JSON response.
#[derive(Deserialize)]
pub struct ApiResponse<T> {
    #[serde(rename = "MRData")]
    pub mr_data: T,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Constructor {
    pub name: String,
    pub nationality: String,
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
