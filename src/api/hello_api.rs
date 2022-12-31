// demo api handler. Playing with some handler and JSON payloads.

use actix_web::{get, Responder, web, Result};
use serde::{Serialize, Deserialize};
use nanoid::nanoid;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
  pub id: String,
  pub message: String,
  pub time_stamp: String
}

#[get("/ping")]
pub async fn ping_pong() -> impl Responder {
  "Hello World!"
}

#[get("/hello")]
pub async fn hello_message() -> Result<impl Responder> {
  let message = Message {
    id: nanoid!(),
    message: "Hello world".to_owned(),
    // to get time in UTC in ISO-8601 format using Chrono.
    time_stamp: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, true)
  };
  Ok(web::Json(message))
}