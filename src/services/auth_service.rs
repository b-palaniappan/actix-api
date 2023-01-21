use actix_web::web::Data;
use actix_web::HttpResponse;
use argon2::{Config, ThreadMode, Variant, Version};
use chrono::Utc;
use log::error;
use mongodb::Client;
use nanoid::nanoid;

use crate::{
    api::auth_api::{LoginRequest, RegisterRequest, RegisterResponse},
    models::auth_model::Auth,
    models::error_model::ApiErrorType,
    repository::auth_repo,
};

pub async fn create_user(
    client: &Data<Client>,
    register_user: RegisterRequest,
) -> Result<HttpResponse, ApiErrorType> {
    // Step 1: Hash password with argon2.
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

    // Step 2: Verify user email does not already exists.
    if auth_repo::check_email(client, &register_user.email).await {
        // Step 3: Store user to MongoDB.
        let current_time = Utc::now();
        let user = Auth {
            id: nanoid!(),
            first_name: register_user.first_name,
            last_name: register_user.last_name,
            email: register_user.email,
            active: true,
            reset_password: false,
            password_hash: match hash {
                Ok(pwd_hash) => pwd_hash,
                Err(_) => return Err(ApiErrorType::InternalServerError),
            },
            created_ts: current_time,
            updated_ts: current_time,
        };

        let registered_user = auth_repo::auth_register(client, user).await;
        match registered_user {
            // User Registered successfully.
            Ok(_) => Ok(HttpResponse::Created().json(RegisterResponse {
                status: "Success".to_owned(),
                message: "User registered successfully".to_owned(),
            })),
            // Internal Server Error.
            Err(err) => {
                error!("Error: {}", err);
                Err(ApiErrorType::InternalServerError)
            }
        }
    } else {
        // User with email already exists.
        Ok(HttpResponse::BadRequest().json(RegisterResponse {
            status: "Failed".to_owned(),
            message: "User already exists with email".to_owned(),
        }))
    }
}

// Login with credentials and generate JWT token after successful login.
pub async fn login(
    client: &Data<Client>,
    login_request: LoginRequest,
) -> Result<HttpResponse, ApiErrorType> {
    // Step 1: Get auth user from MongoDB by email id.
    let auth_user = auth_repo::fetch_by_email(client, &login_request.password).await;
    // Step 2: Check password with hashed password from Database.
    // TODO: remove unwrap from the auth_user...
    let pwd_match = argon2::verify_encoded(
        &auth_user.unwrap().password_hash,
        login_request.password.as_bytes(),
    );
    match pwd_match {
        Ok(_) => {
            // Credentials verified successfully.
            Ok(HttpResponse::Ok().json("Success"))
        }
        Err(_) => Err(ApiErrorType::AuthenticationError),
    }
    // Step 3: Generate JWT token with auth information.
}
