use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put, web,
    web::{Data, Json, Path},
    HttpResponse,
};
use log::info;
use log::warn;
use validator::Validate;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    cfg.service(get_user);
    cfg.service(update_user);
    cfg.service(delete_user);
    cfg.service(get_all_users);
}

#[post("/users")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let is_valid = new_user.validate();
    match is_valid {
        Ok(_) => {
            info!("creating a new user with name - {}", new_user.name);
            let data = User {
                id: None,
                name: new_user.name.to_owned(),
                location: new_user.location.to_owned(),
                title: new_user.title.to_owned(),
            };
            let user_detail = db.create_user(data).await;
            match user_detail {
                Ok(user) => HttpResponse::Created().json(user),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

#[get("/users/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        warn!("User with id -{} not found for get user by ID", id);
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&id).await;
    match user_detail {
        Ok(user) => {
            match user {
                Some(u) => HttpResponse::Created().json(u),
                None => HttpResponse::NotFound().body("No user found with specified ID"),
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/users/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = User {
        id: Some(String::from(&id)),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    let update_result = db.update_user(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;
                match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                }
            } else {
                warn!("User with id -{} not found update user by ID", id);
                HttpResponse::NotFound().body("No user found with specified ID")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_user(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                HttpResponse::NoContent().finish()
            } else {
                warn!("User with id -{} not found for delete user by ID", id);
                HttpResponse::NotFound().body("User with specified ID not found!")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn get_all_users(db: Data<MongoRepo>) -> HttpResponse {
    let users = db.get_all_users().await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
