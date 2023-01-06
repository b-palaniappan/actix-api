// demo api handler. Playing with some handler and JSON payloads.
use actix_web::{get, web, HttpResponse, Responder, Result};
use chrono::Utc;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(ping_pong);
    cfg.service(hello_message);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub message: String,
    pub time_stamp: String,
}

// Ping Pong controller
#[get("/ping")]
pub async fn ping_pong() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

// Hello JSON controller
#[get("/hello")]
pub async fn hello_message() -> Result<impl Responder> {
    let message = Message {
        id: nanoid!(),
        message: "Hello world".to_owned(),
        // to get time in UTC in ISO-8601 format using Chrono.
        time_stamp: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, true),
    };
    Ok(web::Json(message))
}
