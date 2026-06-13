// handlers.rs

use crate::auth::{generate_token, validate_token};
use crate::config;
use crate::db;
use crate::db::{find_or_create_user, init_user_table};
use crate::errors::AppError;
use crate::models::{Article, FinalData, GoogleUser};

use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl, basic::BasicClient,
};
use serde::Deserialize;
use serde_json::json;

// Define a struct to hold the query parameters.
// `serde::Deserialize` allows Actix to automatically parse the query string
// into this struct.
#[derive(Deserialize)]
struct ArticleQuery {
    year: u32,
    month: u32,
    day: u32,
}

// pub fn init_routes(cfg: &mut web::ServiceConfig) {
//     // Scope the API under `/api/v1` for versioning.
//     cfg.service(web::scope("/api/v1").service(get_articles_v1));
// }

// This is for testing the server is up and running
#[get("/Health_Check")]
async fn index(_req: HttpRequest) -> impl Responder {
    // log the request for debugging
    log::info!("Health check endpoint called");
    // Simple JSON response indicating the API is running
    web::Json(json!({
        "message": "Welcome to my API!",
        "status": "ok"
    }))
}

// The new endpoint is `/api/v1/articles?year=...&month=...&day=...`
// This endpoint requires authentication via JWT token in the Authorization header
#[get("/articles")]
async fn get_articles_v1(
    query: web::Query<ArticleQuery>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    // Extract token from Authorization header
    let token: &str = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::AuthError("Missing authorization header".to_string()))?;

    // Validate token and get claims
    let claims: crate::auth::Claims = validate_token(token)?;

    // Log who is requesting articles
    log::info!(
        "Articles requested by {} (Google ID: {}) for date: {}-{}-{}",
        claims.email,
        claims.sub,
        query.year,
        query.month,
        query.day
    );

    // The resource path is `/articles`, but your database table is named `leseco`.
    // We'll pass "leseco" to the database function to match your actual table name.
    // For better long-term consistency, you might consider renaming the table to `articles`.
    let articles: Vec<Article> = db::get_articles("leseco", query.year, query.month, query.day)?;

    let final_data: FinalData = FinalData {
        total_results: articles.len(),
        data_results: articles,
    };

    // for debugging purposes, log the final data
    log::info!("Final Data: {:?}", &final_data);

    // Return the final data as a JSON response
    // Actix will automatically serialize `FinalData` to JSON.
    Ok(HttpResponse::Ok().json(final_data))
}

// authentication handlers will go here in the future
#[derive(Deserialize)]
pub struct AuthCallbackQuery {
    code: String,
    state: String,
}

// Initialize all routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(index)
            .service(get_articles_v1)
            .service(google_auth)
            .service(auth_callback)
            .service(protected_endpoint),
    );
}

// Initialize OAuth2 client
fn build_client() -> BasicClient {
    let config = config::Config::from_env().clone();

    // Set up the OAuth2 client
    BasicClient::new(
        ClientId::new(config.google_client_id),
        Some(ClientSecret::new(config.google_client_secret)),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL"),
        Some(
            TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                .expect("Invalid token endpoint URL"),
        ),
    )
    .set_redirect_uri(RedirectUrl::new(config.google_redirect_url).expect("Invalid redirect URL"))
}

// Google OAuth2 Authorization Endpoint
#[get("/auth/google")]
async fn google_auth() -> Result<impl Responder, AppError> {
    // Build the OAuth2 client

    let client = build_client();

    // log the Build OAuth2 client for debugging
    log::info!("OAuth2 Client built: {:?}", client);

    // Generate the authorization URL to which we'll redirect the user
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(vec![
            Scope::new("email".to_string()),
            Scope::new("profile".to_string()),
        ])
        .url();

    // In production, you'd store csrf_token in a secure session
    Ok(HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish())
}
// Callback handler for Google OAuth2
#[get("/auth/callback")]
async fn auth_callback(query: web::Query<AuthCallbackQuery>) -> Result<impl Responder, AppError> {
    // Build the OAuth2 client
    let client = build_client();
    // log the OAuth2 client for debugging
    log::info!("OAuth2 Client in callback: {:?}", client);

    // Exchange the authorization code for an access token
    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| AppError::AuthError(format!("Token exchange failed: {}", e)))?;

    // Get user info from Google
    let user_info: GoogleUser = reqwest::Client::new()
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token_result.access_token().secret())
        .send()
        .await
        .map_err(|e| AppError::AuthError(format!("Failed to get user info: {}", e)))?
        .json()
        .await
        .map_err(|e| AppError::AuthError(format!("Failed to parse user info: {}", e)))?;

    // Initialize user table and find/create user
    init_user_table()?;
    log::info!("User table initialized");

    let user: crate::models::User = find_or_create_user(&user_info)?;

    // log the user info for debugging
    log::info!("Created User Account: {:?}", user);

    // Generate JWT token
    let token: String = generate_token(&user_info)?;

    // Get frontend URL from config
    let config: config::Config = config::Config::from_env();

    // Redirect to frontend auth callback page with token
    let redirect_url: String = format!("{}/auth/callback?token={}", config.frontend_url, token);

    // log the redirect URL for debugging
    log::info!("Redirecting to frontend: {}", redirect_url);

    Ok(HttpResponse::Found()
        .append_header(("Location", redirect_url))
        .finish())
}

// Protected endpoint example
#[get("/protected")]
async fn protected_endpoint(req: HttpRequest) -> Result<impl Responder, AppError> {
    // Extract token from Authorization header
    let token: &str = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::AuthError("Missing authorization header".to_string()))?;

    // Validate token and get claims
    let claims: crate::auth::Claims = validate_token(token)?;

    // log the claims for debugging
    log::info!("Claims from token: {:?}", claims);

    // Return a success message
    Ok(HttpResponse::Ok().json(json!({
        "message": "Access granted to protected resource",
        "user": claims.email
    })))
}
