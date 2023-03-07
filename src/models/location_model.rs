use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub ip: String,
    pub country: String,
    pub country_iso: String,
    pub region_name: String,
    pub region_code: String,
    pub zip_code: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub time_zone: String,
    pub hostname: String,
}
