use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    #[serde(rename = "_id")]
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    // Password hash using Argon2
    pub password_hash: String,
    // User roles
    pub roles: Vec<String>,
    pub active: bool,
    pub reset_password: bool,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_ts: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_ts: DateTime<Utc>,
}
