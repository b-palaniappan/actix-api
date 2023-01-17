use crate::{
    models::{error_model::ApiErrorType, user_model::User},
    repository::user_repo,
    services::user_service,
};
use actix_web::{
    delete, get, post, put, web,
    web::{Data, Json, Path},
    HttpResponse,
};
use log::error;
use log::warn;
use mongodb::Client;
use validator::Validate;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    cfg.service(get_user);
    cfg.service(update_user);
    cfg.service(delete_user);
    cfg.service(get_all_users);
}

#[post("/users")]
pub async fn create_user(
    client: Data<Client>,
    new_user: Json<User>,
) -> Result<HttpResponse, ApiErrorType> {
    let is_valid = new_user.validate();
    match is_valid {
        Ok(_) => user_service::create_user(&client, new_user).await,
        Err(err) => {
            warn!("Error: {}", err);
            // Validation error.
            Err(ApiErrorType::BadRequest)
        }
    }
}

#[get("/users/{id}")]
pub async fn get_user(
    client: Data<Client>,
    path: Path<String>,
) -> Result<HttpResponse, ApiErrorType> {
    user_service::get_user_by_id(&client, path).await
}

#[put("/users/{id}")]
pub async fn update_user(
    client: Data<Client>,
    path: Path<String>,
    update_user: Json<User>,
) -> Result<HttpResponse, ApiErrorType> {
    user_service::update_user(&client, path, update_user).await
}

#[delete("/users/{id}")]
pub async fn delete_user(
    client: Data<Client>,
    path: Path<String>,
) -> Result<HttpResponse, ApiErrorType> {
    user_service::delete_user(&client, path).await
}

#[get("/users")]
pub async fn get_all_users(client: Data<Client>) -> Result<HttpResponse, ApiErrorType> {
    user_service::get_all_users(&client).await
}
