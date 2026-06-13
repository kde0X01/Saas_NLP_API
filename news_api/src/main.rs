// Simple API objectif is to serve an Archived News Papers Data.

mod auth;
mod config;
mod db;
mod errors;
mod handlers;
mod models;

// main.rs
use crate::config::Config;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
// use serde_json::json;
// use config;
use std::io::{self, ErrorKind, Write};
use std::path::Path;
use std::str::FromStr;

// main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Try to load config from environment, fall back to defaults
    // (env vars: BIND_ADDR, TLS_KEY, TLS_CERT, LOG_LEVEL)
    // Load .env file in development. Fails silently if not found.
    // dotenv::dotenv().ok();
    // let config: config::Config = crate::config::CONFIG.clone();

    // load the config from the .env file
    let config: Config = envy::from_env::<Config>().unwrap_or_default();
    // test logging of loaded config
    // log::info!("Loaded configuration: {:?}", config);

    // let config = envy::from_env::<Config::CONFIG>().unwrap_or_default();

    // set up logger using configured level
    let level: LevelFilter =
        LevelFilter::from_str(&config.log_level.to_lowercase()).unwrap_or(LevelFilter::Info);
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, level)
        .init();

    log::info!("Loading TLS Keys ...");

    let addr: String = config::Config::from_env().bind_addr;
    let key_path: String = config::Config::from_env().tls_key;
    let cert_path: String = config::Config::from_env().tls_cert;

    // quick checks with informative errors
    if !Path::new(&key_path).exists() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            format!("Private key not found at '{}'", key_path),
        ));
    }

    // check cert path
    if !Path::new(&cert_path).exists() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            format!("Certificate chain not found at '{}'", cert_path),
        ));
    }

    // set up SSL builder
    let mut builder: openssl::ssl::SslAcceptorBuilder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls())
            .map_err(|e| io::Error::other(format!("Failed to create SSL acceptor: {}", e)))?;
    builder
        .set_private_key_file(&key_path, SslFiletype::PEM)
        .map_err(|e| io::Error::other(format!("Failed to load key: {}", e)))?;
    builder
        .set_certificate_chain_file(&cert_path)
        .map_err(|e| io::Error::other(format!("Failed to load cert chain: {}", e)))?;

    // log server start
    log::info!("starting HTTPS server at https://{}", addr);
    // HttpServer::new(|| App::new().wrap(Logger::default()).service(index))
    // Start the HTTP server with TLS
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8000") // Your Yew app's origin
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .max_age(3600);
        // Create the Actix web application with CORS and routes
        App::new().wrap(cors).configure(handlers::init_routes)
    })
    .bind_openssl(&addr, builder)?
    .run()
    .await
}

// TODO:
// 1. Add Google Authentication
// 2. Build A Web Client that connect to the Back-end after a Successful Authentication.
// 3. Add Logging. ( To the console First , than to A table for a Book Keeping).
// 4. Save The Logs to A file as well.
