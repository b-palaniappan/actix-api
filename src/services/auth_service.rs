use actix_web::web::Data;
use actix_web::HttpResponse;
use argon2::{Config, Variant, Version};
use chrono::Utc;
use log::{error, warn};
use mongodb::Client;
use nanoid::nanoid;
use rand::Rng;

use crate::auth::claims::Claims;
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
    // Generate a random 16-byte salt using the rand crate
    let mut rng = rand::rng();
    let salt: [u8; 16] = rng.random();
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 10,
        lanes: 4,
        secret: &[],
        ad: &[],
        hash_length: 64,
    };
    let hash = argon2::hash_encoded(register_user.password.as_bytes(), &salt, &config);

    // Step 2: Verify user email does not already exists.
    if auth_repo::check_email(client, &register_user.email).await {
        // Step 3: Store user to MongoDB.
        let current_time = Utc::now();
        let user = Auth {
            id: nanoid!(),
            first_name: register_user.first_name,
            last_name: register_user.last_name,
            email: register_user.email,
            roles: vec![String::from("ROLE_USER")],
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
    let auth_user = auth_repo::fetch_by_email(client, &login_request.email).await;

    // Check if the user
    match &auth_user {
        Some(a) => {
            // Step 2: Check password with hashed password from Database.
            let pwd_match =
                argon2::verify_encoded(&a.password_hash, login_request.password.as_bytes())
                    .unwrap();
            if pwd_match {
                // Credentials verified successfully.
                // Step 3: Generate JWT token with auth information.
                match Claims::create_jwt_token(a) {
                    Ok(response) => Ok(HttpResponse::Ok().json(response)),
                    Err(_) => Err(ApiErrorType::AuthenticationError),
                }
            } else {
                Err(ApiErrorType::InvalidCredential)
            }
        }
        None => {
            warn!("User not found for email {}", login_request.email);
            Err(ApiErrorType::InvalidCredential)
        }
    }
}
