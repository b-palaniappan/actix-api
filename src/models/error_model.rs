use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display)]
pub enum ApiErrorType {
    #[display(fmt = "Internal server error. Try again after some time.")]
    InternalServerError,

    #[display(fmt = "Bad request. Missing parameter or wrong payload.")]
    BadRequest,

    #[display(fmt = "Unsupported media type. Only support JSON")]
    UnsupportedMediaType,

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

// Struct which defines the structure of the error response
#[derive(Serialize)]
struct FormattedErrorResponse {
    status_code: u16,
    error: String,
    message: String,
}

#[derive(Debug, Display, Error)]
pub enum CustomError {
    // Formatting the validation error message
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },

    // Formatting the internal server error message
    #[display(fmt = "An internal error occured. Please try again later.")]
    InternalError,

    // Formatting the bad request error message
    #[display(fmt = "Bad request")]
    BadClientData,

    // Formatting the Not found error message
    #[display(fmt = "Not found")]
    NotFound,
}

impl CustomError {
    fn name(&self) -> String {
        match self {
            CustomError::ValidationError { .. } => "Validation Error".to_owned(),
            CustomError::InternalError => "Internal Server Error".to_owned(),
            CustomError::BadClientData => "Bad request".to_owned(),
            CustomError::NotFound => "Not found".to_owned(),
        }
    }
}

// Implementation ResponseError trait for the custom struct
impl ResponseError for CustomError {
    // Function to generate the error response
    fn error_response(&self) -> HttpResponse {
        let error_response = FormattedErrorResponse {
            status_code: self.status_code().as_u16(),
            error: self.to_string(),
            message: self.to_string(),
        };
        HttpResponse::build(self.status_code()).json(error_response)
    }
    // Function to generate error code
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            CustomError::BadClientData => StatusCode::BAD_REQUEST,
            CustomError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

// impl ResponseError for ApiError {
// fn status_code(&self) -> StatusCode {}

// fn error_response(&self) -> HttpResponse {
// match self {
// ApiError::InternalServerError => {
// Ok(HttpResponse::InternalServerError().json(ApiError {
// status: 500,
// time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
// message: "Internal server error".to_owned,
// debug_message: None,
// }));
// }

// ApiError::BadRequest => {
// Ok(HttpResponse::BadRequest().json(ApiError {
// status: 400,
// time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
// message: "Bad request".to_owned(),
// debug_message: Some("Bad request parameter or payload.".to_owned()),
// }));
// }

// ApiError::UserNotFound => {
// Ok(HttpResponse::NotFound().json(ApiError {
// status: 404,
// time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
// message: "User not found for the given id".to_owned(),
// debug_message: "User not found for the given id. Verify the user unique id in the request."
// }));
// }

// _ => Ok(HttpResponse::InternalServerError().json(ApiError {
// status: 500,
// time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
// message: "Internal server error".to_owned(),
// debug_message: None,
// })),
// }
// }
// }

