use crate::models::error_model::ApiErrorType;
use actix_web::web::Data;
use actix_web::HttpResponse;
use mongodb::Client;

pub async fn create_user(client: &Data<Client>) -> Result<HttpResponse, ApiErrorType> {
    // Step 1: Hash password with argon2.
    // Step 2: Store user to MongoDB.
    Ok(_)
}
