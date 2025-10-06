mod api;
mod models;

pub mod stats;

// Hide internal structure of models and group everything under one data module.
pub mod data {
    pub use crate::models::common::{ApiResponse, Constructor, Driver};
    pub use crate::models::{constructor_standings, driver_standings, drivers, race_results};
}

pub use crate::api::AppResult;
pub use crate::api::fetch_constructor_standings;
pub use crate::api::fetch_driver_results;
pub use crate::api::fetch_driver_standings;
pub use crate::api::fetch_drivers;
pub use crate::api::fetch_race_results;
