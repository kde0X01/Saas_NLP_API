// utils.rs - Utility functions for authentication and API calls

use serde::{Deserialize, Serialize};
use web_sys::window;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub email: String,
}

// Get the auth token from localStorage
pub fn get_token() -> Option<String> {
    if let Some(win) = window() {
        if let Ok(Some(storage)) = win.local_storage() {
            if let Ok(Some(token)) = storage.get_item("auth_token") {
                return Some(token);
            }
        }
    }
    None
}

// Get user email from localStorage
pub fn get_user_email() -> Option<String> {
    if let Some(win) = window() {
        if let Ok(Some(storage)) = win.local_storage() {
            if let Ok(Some(email)) = storage.get_item("user_email") {
                return Some(email);
            }
        }
    }
    None
}

// Store user email in localStorage
pub fn set_user_email(email: &str) {
    if let Some(win) = window() {
        if let Ok(Some(storage)) = win.local_storage() {
            let _ = storage.set_item("user_email", email);
        }
    }
}

// Clear auth data from localStorage
pub fn clear_auth() {
    if let Some(win) = window() {
        if let Ok(Some(storage)) = win.local_storage() {
            let _ = storage.remove_item("auth_token");
            let _ = storage.remove_item("user_email");
        }
    }
}

// Check if user is authenticated
pub fn is_authenticated() -> bool {
    get_token().is_some()
}
