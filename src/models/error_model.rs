use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use chrono::{SecondsFormat, Utc};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::ValidationErrors;

// TODO: Handle validation error with whats wrong and which filed...
// -- Error handing.
#[derive(Debug, Display, Error)]
pub enum ApiErrorType {
    #[display(fmt = "Internal server error. Try again after some time.")]
    InternalServerError,

    #[display(fmt = "Bad request.")]
    BadRequest,

    #[display(fmt = "User not found for the given ID")]
    UserNotFound,

    #[display(fmt = "Authentication error.")]
    AuthenticationError,

    #[display(fmt = "Authorization error.")]
    AuthorizationError,

    #[display(fmt = "Validation error on field")]
    ValidationError {
        validation_error: ValidationErrors,
        object: String,
    },
}

#[derive(Debug, Serialize)]
pub struct ValidationError {
    object: String,
    field: String,
    rejected_value: String,
    message: String,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub status: u16,
    pub time: String,
    pub message: String,
    pub debug_message: Option<String>,
    pub sub_errors: Vec<ValidationError>,
}

// Set Debug Error messages for Global error.
impl ApiErrorType {
    fn debug_message(&self) -> String {
        match self {
            ApiErrorType::InternalServerError => {
                "Internal server error. Please try again later.".to_owned()
            }
            ApiErrorType::BadRequest => {
                "Bad request. Missing parameter or wrong payload.".to_owned()
            }
            ApiErrorType::UserNotFound => "User not found for given ID".to_owned(),
            ApiErrorType::AuthenticationError => {
                "User not authenticated. Please reauthenticate and try again.".to_owned()
            }
            ApiErrorType::AuthorizationError => {
                "User not authorized to access this resource.".to_owned()
            }
            ApiErrorType::ValidationError { .. } => "Validation error".to_owned(),
        }
    }
}

// Global error handling with actix-web ResponseError.
impl ResponseError for ApiErrorType {
    // Global error handler status code.
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiErrorType::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorType::BadRequest => StatusCode::BAD_REQUEST,
            ApiErrorType::UserNotFound => StatusCode::NOT_FOUND,
            ApiErrorType::AuthenticationError => StatusCode::UNAUTHORIZED,
            ApiErrorType::AuthorizationError => StatusCode::FORBIDDEN,
            ApiErrorType::ValidationError { .. } => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    // Global error handler Http Response payload
    fn error_response(&self) -> HttpResponse {
        let mut validation_sub_errs = vec![];
        match self {
            // Iterate thru validation error object
            ApiErrorType::ValidationError {
                validation_error,
                object,
            } => {
                for (field, field_errors) in validation_error.field_errors() {
                    for field_error in field_errors {
                        validation_sub_errs.push(ValidationError {
                            object: object.to_string(),
                            field: field.to_owned(),
                            rejected_value: field_error.params.get("value").unwrap().to_string(),
                            message: field_error.message.as_ref().unwrap().to_string(),
                        })
                    }
                }
            }
            _ => {
                validation_sub_errs = vec![];
            }
        }
        HttpResponse::build(self.status_code()).json(ApiError {
            status: self.status_code().as_u16(),
            time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
            message: self.to_string(),
            debug_message: Some(self.debug_message()),
            sub_errors: validation_sub_errs,
        })
    }
}
