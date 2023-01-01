use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[validate(length(min = 2, message = "Name must be have minimum of 3 characters"))]
    pub name: String,
    #[validate(length(
        min = 2,
        max = 15,
        message = "Location character length between 2 and 15"
    ))]
    pub location: String,
    pub title: String,
}
