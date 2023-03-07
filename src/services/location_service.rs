use actix_web::HttpResponse;
use log::error;

use crate::models::error_model::ApiErrorType;
use crate::models::location_model::Location;

pub async fn get_location() -> Result<HttpResponse, ApiErrorType> {
    match ip_loc().await {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => {
            error!("Error calling REST API: {}", e);
            Err(ApiErrorType::InternalServerError)
        }
    }
}

async fn ip_loc() -> Result<Location, reqwest::Error> {
    let response = reqwest::get("https://ifconfig.co/json")
        .await?
        .json::<Location>()
        .await?;

    Ok(response)
}
