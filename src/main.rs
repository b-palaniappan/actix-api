mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use api::user_api::{create_user, get_user, update_user};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}