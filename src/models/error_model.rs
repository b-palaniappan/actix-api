use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
  pub status: String,
  pub time: String,
  pub message: String,
  pub debug_message: String,

}