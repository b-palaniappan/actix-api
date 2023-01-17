use crate::models::error_model::ApiErrorType;
use crate::models::user_model::User;
use crate::repository::user_repo;
use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;
use log::{error, warn};
use mongodb::Client;

// add a new user to MongoDB
pub async fn create_user(
    client: &Data<Client>,
    new_user: Json<User>,
) -> Result<HttpResponse, ApiErrorType> {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    let user_detail = user_repo::create_user(&client, data).await;
    match user_detail {
        Ok(user) => Ok(HttpResponse::Created().json(user)),
        Err(err) => {
            error!("Error: {}", err);
            Err(ApiErrorType::InternalServerError)
        }
    }
}

pub async fn get_user_by_id(
    client: &Data<Client>,
    path: Path<String>,
) -> Result<HttpResponse, ApiErrorType> {
    let id = path.into_inner();
    if id.is_empty() {
        warn!("User with id - {} not found for get user by ID", id);
        return Err(ApiErrorType::BadRequest);
    }
    let user_detail = user_repo::get_user(&client, &id).await;
    match user_detail {
        Ok(Some(user)) => Ok(HttpResponse::Created().json(user)),
        Ok(None) => Err(ApiErrorType::UserNotFound),
        Err(err) => {
            error!("Error: {}", err);
            Err(ApiErrorType::InternalServerError)
        }
    }
}

pub async fn update_user(
    client: &Data<Client>,
    path: Path<String>,
    update_user: Json<User>,
) -> Result<HttpResponse, ApiErrorType> {
    let id = path.into_inner();
    if id.is_empty() {
        return Err(ApiErrorType::BadRequest);
    };
    let data = User {
        id: Some(String::from(&id)),
        name: update_user.name.to_owned(),
        location: update_user.location.to_owned(),
        title: update_user.title.to_owned(),
    };
    let update_result = user_repo::update_user(&client, &id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = user_repo::get_user(&client, &id).await;
                match updated_user_info {
                    Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
                    Ok(None) => Err(ApiErrorType::UserNotFound),
                    Err(err) => {
                        error!("Error: {}", err);
                        Err(ApiErrorType::InternalServerError)
                    }
                }
            } else {
                warn!("User with id -{} not found update user by ID", id);
                Err(ApiErrorType::UserNotFound)
            }
        }
        Err(err) => {
            error!("Error: {}", err);
            Err(ApiErrorType::InternalServerError)
        }
    }
}

pub async fn delete_user(
    client: &Data<Client>,
    path: Path<String>,
) -> Result<HttpResponse, ApiErrorType> {
    let id = path.into_inner();
    if id.is_empty() {
        return Err(ApiErrorType::UserNotFound);
    };
    let result = user_repo::delete_user(&client, &id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                Ok(HttpResponse::NoContent().finish())
            } else {
                warn!("User with id -{} not found for delete user by ID", id);
                Err(ApiErrorType::UserNotFound)
            }
        }
        Err(err) => {
            error!("Error : {}", err);
            Err(ApiErrorType::InternalServerError)
        }
    }
}

pub async fn get_all_users(client: &Data<Client>) -> Result<HttpResponse, ApiErrorType> {
    let users = user_repo::get_all_users(&client).await;
    match users {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(err) => {
            error!("Error : {}", err);
            Err(ApiErrorType::InternalServerError)
        }
    }
}
