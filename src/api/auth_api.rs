use actix_web::web::Data;
use actix_web::{get, post, put, web, web::Json, HttpResponse};
use log::info;
use log::warn;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{models::error_model::ApiErrorType, services::auth_service};

// -- configurations
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(auth_register);
    cfg.service(auth_login);
    cfg.service(update_password);
    cfg.service(forgot_password);
    cfg.service(logout);
}

// -- DTO's
// TODO: Move DTO's to models or different package.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "email must be valid email"))]
    pub email: String,

    #[validate(length(min = 1, max = 50, message = "first name length between 1 and 50"))]
    pub first_name: String,

    #[validate(length(min = 2, max = 50, message = "last name length between 2 and 50"))]
    pub last_name: String,

    #[validate(length(
        min = 12,
        message = "password is required and must be at least 12 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "email must be valid email"))]
    pub email: String,

    #[validate(length(
        min = 12,
        message = "password is required and must be at least 12 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePasswordRequest {
    #[validate(email(message = "email must be valid email"))]
    pub email: String,

    #[validate(length(
        min = 12,
        message = "current password is required and must be at least 12 characters"
    ))]
    pub current_password: String,

    #[validate(length(
        min = 12,
        message = "new password is required and must be at least 12 characters"
    ))]
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "email must be valid email"))]
    pub email: String,
}

// -- Controllers...
// Register a user.
#[post("/a/register")]
pub async fn auth_register(
    client: Data<Client>,
    register_user: Json<RegisterRequest>,
) -> Result<HttpResponse, ApiErrorType> {
    // Step 1: Validate payload.
    match register_user.validate() {
        Ok(_) => auth_service::create_user(&client, register_user.0).await,
        Err(err) => {
            warn!("Error: {}", err);
            // Validation error.
            Err(ApiErrorType::BadRequest)
        }
    }
}

// Login using credentials.
#[post("/a/login")]
pub async fn auth_login(
    client: Data<Client>,
    login_user: Json<LoginRequest>,
) -> Result<HttpResponse, ApiErrorType> {
    // Step 1: Validate payload.
    match login_user.validate() {
        Ok(_) => auth_service::login(&client, login_user.0).await,
        Err(err) => {
            warn!("Error: {}", err);
            Err(ApiErrorType::BadRequest)
        }
    }
}

// Update password for an existing user with credentials.
#[put("/a/password")]
pub async fn update_password(update_password: Json<UpdatePasswordRequest>) -> HttpResponse {
    HttpResponse::Ok().json(update_password)
}

// Forgett password flow.
#[post("/a/forgot-password")]
pub async fn forgot_password(forgot_password: Json<ForgotPasswordRequest>) -> HttpResponse {
    HttpResponse::Ok().json(forgot_password)
}

// Logout user.
#[get("/a/logout")]
pub async fn logout(path: web::Path<String>) -> HttpResponse {
    info!("{}", path);
    HttpResponse::NoContent().finish()
}
