//
// register module
// pages/register.rs
use dioxus::prelude::*;

#[component]
pub fn RegisterPage() -> Element {
    // Auth URL for Google OAuth
    let auth_url: &str = "https://127.0.0.1:8080/api/v1/auth/google";

    rsx! {
        div {
            padding: "2rem",
            h2 { "Register using Google" }
            p { "Click to create or register an account using your Google identity." }
            a {
                href: "{auth_url}",
                button {
                    style: "padding: 10px 16px; background:#34A853; color: white; border:none; border-radius:6px; cursor:pointer;",
                    "Register with Google"
                }
            }
        }
    }
}
