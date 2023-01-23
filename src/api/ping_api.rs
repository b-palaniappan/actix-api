use actix_web::{get, web, HttpResponse};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}

// Ping controller to check the health of application.
#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong!".to_string())
}
