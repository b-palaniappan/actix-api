use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::api::auth_api::LoginResponse;
use crate::models::auth_model::Auth;

// JWT lifetime and Secret key.
const JWT_EXPIRATION_HOURS: i64 = 24;
const SECRET: &str = "secret";

// Claims for JWT Body.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub permissions: Vec<String>,
    exp: i64,
    iat: i64,
}

// Kind of constructor for Rust.
impl Claims {
    pub fn new(sub: &String, permissions: &Vec<String>) -> Self {
        Self {
            sub: sub.to_string(),
            permissions: permissions.to_owned(),
            exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp(),
            iat: (Utc::now()).timestamp(),
        }
    }

    // Create JWT token from Auth values.
    pub fn create_jwt_token(auth: &Auth) -> Result<LoginResponse, Error> {
        let claim = Claims::new(&auth.id, &auth.roles);
        let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());
        let jwt_token = jsonwebtoken::encode(&Header::default(), &claim, &encoding_key);
        match jwt_token {
            Ok(token) => Ok(LoginResponse {
                access_token: token,
                token_type: "Bearer".to_string(),
            }),
            Err(e) => Err(ErrorUnauthorized(e)),
        }
    }

    // Decode JWT and validate signature
    pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
        let decoding_key = DecodingKey::from_secret(SECRET.as_bytes());
        jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| ErrorUnauthorized(e.to_owned()))
    }
}
