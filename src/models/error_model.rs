use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use chrono::{SecondsFormat, Utc};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

// TODO: Handle validation error with whats wrong and which filed...
// -- Error handing.
#[derive(Debug, Display, Error)]
pub enum ApiErrorType {
    #[display(fmt = "Internal server error. Try again after some time.")]
    InternalServerError,

    #[display(fmt = "Bad request. Missing parameter or wrong payload.")]
    BadRequest,

    #[display(fmt = "User not found for the given ID")]
    UserNotFound,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub status: u16,
    pub time: String,
    pub message: String,
    pub debug_message: Option<String>,
}

// Set Debug Error messages for Global error.
impl ApiErrorType {
    fn debug_message(&self) -> String {
        match self {
            ApiErrorType::InternalServerError => {
                "Internal server error. Please try again later.".to_owned()
            }
            ApiErrorType::BadRequest => "User not found for the given ID".to_owned(),
            ApiErrorType::UserNotFound => "User not found for given ID".to_owned(),
        }
    }
}

// Global error handling with actix-web ResponseError.
impl ResponseError for ApiErrorType {
    // Global error handler Http Response payload
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ApiError {
            status: self.status_code().as_u16(),
            time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
            message: self.to_string(),
            debug_message: Some(self.debug_message()),
        })
    }

    // Global error handler status code.
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiErrorType::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorType::BadRequest => StatusCode::BAD_REQUEST,
            ApiErrorType::UserNotFound => StatusCode::NOT_FOUND,
        }
    }
}
