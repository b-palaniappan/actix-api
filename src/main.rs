mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users};
use repository::mongodb_repo::MongoRepo;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    let host = env::var("SERVER.HOST").expect("Missing server Host");
    let port: u16 = env::var("SERVER.PORT").expect("Missing server Port number").parse().unwrap_or(8080);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
    })
    .bind((host, port))?
    .run()
    .await
}
