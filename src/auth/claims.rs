use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// JWT lifetime and Secret key.
const JWT_EXPIRATION_HOURS: i64 = 24;
const SECRET: &str = "secret";

// Claims for JWT Body.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub permissions: Vec<String>,
    exp: i64,
}

// Kind of constructor for Rust.
impl Claims {
    pub fn new(email: String, permissions: Vec<String>) -> Self {
        Self {
            email,
            permissions,
            exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp(),
        }
    }
}

// Create a JWT.
pub fn create_jwt(claims: Claims) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| ErrorUnauthorized(e.to_owned()))
}

// Decode JWT.
pub fn decode_jet(token: &str) -> Result<Claims, Error> {
    let decoding_key = DecodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ErrorUnauthorized(e.to_owned()))
}

