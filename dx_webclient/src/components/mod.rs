//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals.

mod auth;
pub use auth::AuthProvider;

mod check;
pub use check::Check;

mod auth_callback;
pub use auth_callback::AuthCallbackPage;

mod utils;
pub use utils::{clear_auth, get_token, get_user_email, is_authenticated, set_user_email};

mod logout;
pub use logout::Logout;
