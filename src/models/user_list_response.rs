use crate::models::user_model::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub href: String,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub limit: i64,
    pub offset: u64,
    pub total: u64,
    pub size: usize,
    pub items: Vec<User>,
}
