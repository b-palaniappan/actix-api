mod api;
mod models;
mod repository;

use actix_web::{
    error::Error, error::InternalError, error::JsonPayloadError, HttpRequest, HttpResponse,
};
use actix_web::{middleware, web::Data, web::JsonConfig, App, HttpServer};
use dotenv::dotenv;
use log::info;
use repository::mongodb_repo::MongoRepo;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize Log4rs
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Initializing application...");

    // Load dot env file
    dotenv().ok();

    // Initialize MongoDB connection
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    // Get Server host and port number from environment file.
    let server_host = match env::var("SERVER.HOST") {
        Ok(v) => v.to_string(),
        Err(_) => "127.0.0.1".to_string(),
    };

    let server_port: u16 = match env::var("SERVER.PORT") {
        Ok(v) => v.parse().unwrap_or(8080),
        Err(_) => 8080,
    };
    info!(
        "Starting actix-web server in {}:{}",
        server_host, server_port
    );

    // Config and start Actix-web server.
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(db_data.clone())
            .app_data(JsonConfig::default().error_handler(json_error_handler))
            .configure(api::init_user_api)
            .configure(api::init_hello_api)
            .configure(api::init_auth_api)
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
    })
    .bind((server_host, server_port))
    .unwrap_or_else(|_| panic!("error binding to port '{:?}'", stringify!($server_port)))
    .run()
    .await
}

// Handle json parser errors.
fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(detail),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().json(detail)
        }
        _ => HttpResponse::BadRequest().json(detail),
    };
    InternalError::from_response(err, resp).into()
}
