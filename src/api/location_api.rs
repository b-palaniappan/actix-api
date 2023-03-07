use actix_web::{get, HttpResponse, web};
use crate::models::error_model::ApiErrorType;
use crate::services::location_service;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_location);
}

#[get("/locations")]
pub async fn get_location() -> Result<HttpResponse, ApiErrorType> {
    location_service::get_location().await
}
