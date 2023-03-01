use std::env;

use actix_cors::Cors;
use actix_web::dev::ServiceRequest;
use actix_web::{
    error::Error, error::InternalError, error::JsonPayloadError, http, web, HttpRequest,
    HttpResponse,
};
use actix_web::{middleware, web::Data, web::JsonConfig, App, HttpServer};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use chrono::{SecondsFormat, Utc};
use dotenv::dotenv;
use log::{info, warn};

use models::error_model::ApiError;

use crate::auth::claims::Claims;
use crate::config::db;

mod api;
mod auth;
mod config;
mod constants;
mod models;
mod repository;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize Log4rs
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Initializing application...");

    // Load dot env file
    dotenv().ok();

    // Initialize MongoDB connection
    let client = db::init().await;

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
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            // Configure CORS
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_origin("http://localhost:8080")
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            // configure compress middleware
            .wrap(middleware::Compress::default())
            // configure app data
            .app_data(Data::new(client.clone()))
            .app_data(JsonConfig::default().error_handler(json_error_handler))
            // Configure un-secure controller
            .configure(api::init_auth_api)
            .configure(api::init_ping_api)
            // Configure secure controller with prefix '/api'
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .configure(api::init_user_api)
                    .configure(api::init_hello_api),
            )
            // configure controller
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.3.0")))
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
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(ApiError {
            status: 415,
            time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
            message: "Unsupported media type".to_owned(),
            debug_message: Some(detail),
            sub_errors: Vec::new(),
        }),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().json(ApiError {
                status: 422,
                time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
                message: "Unprocessable payload".to_owned(),
                debug_message: Some(detail),
                sub_errors: Vec::new(),
            })
        }
        _ => HttpResponse::BadRequest().json(ApiError {
            status: 400,
            time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
            message: "Bad request. Missing parameter and / or wrong payload.".to_owned(),
            debug_message: Some(detail),
            sub_errors: Vec::new(),
        }),
    };
    InternalError::from_response(err, resp).into()
}

// Validator for JWT Token and extract permissions.
async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    warn!("Validate jwt auth");
    // We just get permissions from JWT
    let result = Claims::decode_jwt(credentials.token());
    match result {
        Ok(claims) => {
            req.attach(claims.permissions);
            Ok(req)
        }
        // required by `actix-web-httpauth` validator signature
        Err(e) => Err((e, req)),
    }
}
