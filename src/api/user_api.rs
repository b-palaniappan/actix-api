use actix_web::{
    delete, get, post, put, web,
    web::{Data, Json, Path},
    HttpResponse,
};
use actix_web_grants::proc_macro::has_any_role;
use log::warn;
use mongodb::Client;
use serde::Deserialize;
use validator::Validate;

use crate::{
    models::{error_model::ApiErrorType, user_model::User},
    services::user_service,
};

// -- Configurations...
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    cfg.service(get_user);
    cfg.service(update_user);
    cfg.service(delete_user);
    cfg.service(get_all_users);
}

// -- Controllers...
// Create a user
#[post("/users")]
pub async fn create_user(
    client: Data<Client>,
    new_user: Json<User>,
) -> Result<HttpResponse, ApiErrorType> {
    let is_valid = new_user.validate();
    match is_valid {
        Ok(_) => user_service::create_user(&client, new_user).await,
        Err(err) => {
            warn!("Payload validation Error on add user: {}", err);
            // Validation error.
            Err(ApiErrorType::ValidationError {
                validation_error: err,
                object: "User".to_string(),
            })
        }
    }
}

// Get user by unique user id.
#[get("/users/{id}")]
pub async fn get_user(
    client: Data<Client>,
    path: Path<String>,
) -> Result<HttpResponse, ApiErrorType> {
    user_service::get_user_by_id(&client, path).await
}

// Update user by unique user id.
#[put("/users/{id}")]
pub async fn update_user(
    client: Data<Client>,
    path: Path<String>,
    update_user: Json<User>,
) -> Result<HttpResponse, ApiErrorType> {
    user_service::update_user(&client, path, update_user).await
}

// Delete user by unique user id.
#[delete("/users/{id}")]
pub async fn delete_user(
    client: Data<Client>,
    path: Path<String>,
) -> Result<HttpResponse, ApiErrorType> {
    user_service::delete_user(&client, path).await
}

#[derive(Deserialize)]
pub struct Pagination {
    pub offset: Option<u64>,
    pub limit: Option<i64>,
}

// Get list of all user in the database.
#[get("/users")]
#[has_any_role("USER")]
pub async fn get_all_users(
    client: Data<Client>,
    pagination: web::Query<Pagination>,
) -> Result<HttpResponse, ApiErrorType> {
    user_service::get_all_users(&client, &pagination.0).await
}
