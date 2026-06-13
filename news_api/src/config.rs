// src/config.rs

// use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;

#[allow(dead_code)]
#[derive(Default, Debug, Deserialize, Clone)]
// Application configuration structure.
pub struct Config {
    pub db_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,
    pub frontend_url: String,
    pub log_level: String,
    pub jwt_secret: String,
    pub(crate) bind_addr: String,
    pub(crate) tls_key: String,
    pub(crate) tls_cert: String,
}

// Implementation to load configuration from environment variables.
impl Config {
    pub fn from_env() -> Self {
        // Load .env file
        dotenv::dotenv().ok();

        let db_url: String =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://db.sqlite".to_string());
        let google_client_id: String =
            env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
        let google_client_secret: String =
            env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
        // Trim surrounding quotes/whitespace to avoid RelativeUrlWithoutBase errors
        let google_redirect_url: String = env::var("GOOGLE_REDIRECT_URL")
            .expect("GOOGLE_REDIRECT_URL must be set")
            .trim_matches(|c| c == '"' || c == '\'')
            .trim()
            .to_string();

        // validate early (optional)
        let _ = url::Url::parse(&google_redirect_url)
            .expect("GOOGLE_REDIRECT_URL is not a valid absolute URL");

        let log_level: String = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        let jwt_secret: String = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let frontend_url: String = env::var("FRONTEND_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
        let bind_addr: String =
            env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let tls_key: String = env::var("TLS_KEY").expect("TLS_KEY must be set");
        let tls_cert: String = env::var("TLS_CERT").expect("TLS_CERT must be set");

        Config {
            db_url,
            google_client_id,
            google_client_secret,
            google_redirect_url,
            frontend_url,
            log_level,
            jwt_secret,
            bind_addr,
            tls_key,
            tls_cert,
        }
    }
}

// DB Config.
// pub const DB_URL: &str = "db/saas_api.db";
pub const DB_URL: &str = "db/db_news.db";
