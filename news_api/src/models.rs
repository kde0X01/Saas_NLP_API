// models.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
// Article Model
pub struct Article {
    pub pub_year: u32,
    pub pub_month: u32,
    pub pub_day: u32,
    pub category: String,
    pub title: String,
    pub link: String,
    pub content: String,
}

// Final Data Model
#[derive(Serialize, Debug)]
// Structure to hold the final response data
pub struct FinalData {
    pub total_results: usize,
    pub data_results: Vec<Article>,
}

// Auth Models
#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUser {
    pub sub: String, // Google's unique user ID
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
    pub email_verified: bool,
}

// Response structure after authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: GoogleUser,
}

// User Model for database operations
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub google_id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}
