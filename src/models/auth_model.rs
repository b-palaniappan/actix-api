use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    #[serde(rename = "_id")]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub email: String,

    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_ts: DateTime<Utc>,
}
