use actix_web::{get, web, HttpResponse};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong!".to_string())
}
