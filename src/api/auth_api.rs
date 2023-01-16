use actix_web::{get, post, put, web, web::Json, HttpResponse};
use argon2::{Config, ThreadMode, Variant, Version};
use log::info;
use log::warn;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::error_model::ApiErrorType;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(auth_register);
    cfg.service(auth_login);
    cfg.service(update_password);
    cfg.service(forgot_password);
    cfg.service(logout);
}

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

#[post("/a/register")]
pub async fn auth_register(
    register_user: Json<RegisterRequest>,
) -> Result<HttpResponse, ApiErrorType> {
    // Step 1: Validate payload.
    // Step 2: Hash password with argon2.
    // Step 3: Store user to MongoDB.
    match register_user.validate() {
        Ok(_) => {
            let salt = b"radomSalt";
            let config = Config {
                variant: Variant::Argon2id,
                version: Version::Version13,
                mem_cost: 65536,
                time_cost: 10,
                lanes: 4,
                thread_mode: ThreadMode::Parallel,
                secret: &[],
                ad: &[],
                hash_length: 64,
            };
            let hash = argon2::hash_encoded(register_user.password.as_bytes(), salt, &config);
            info!("Hash - {}", hash.unwrap());
            Ok(HttpResponse::Ok().json(RegisterResponse {
                status: "Success".to_owned(),
                message: "User registered successfully".to_owned(),
            }))
        }
        Err(err) => {
            warn!("Error: {}", err);
            // Validation error.
            Err(ApiErrorType::BadRequest)
        }
    }
}

#[post("/a/login")]
pub async fn auth_login(login_user: Json<LoginRequest>) -> HttpResponse {
    HttpResponse::Ok().json(login_user)
}

#[put("/a/password")]
pub async fn update_password(update_password: Json<UpdatePasswordRequest>) -> HttpResponse {
    HttpResponse::Ok().json(update_password)
}

#[post("/a/forgot-password")]
pub async fn forgot_password(forgot_password: Json<ForgotPasswordRequest>) -> HttpResponse {
    HttpResponse::Ok().json(forgot_password)
}

#[get("/a/logout/{user_id}")]
pub async fn logout(path: web::Path<String>) -> HttpResponse {
    info!("{}", path);
    HttpResponse::NoContent().finish()
}
