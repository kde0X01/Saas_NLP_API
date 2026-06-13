// Health check against GET /api/v1/Health_Check (see simple_api handlers).

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CheckResponse {
    message: String,
    status: String,
}

// function to check if the backend is UP and Running !!!
#[component]
pub fn Check() -> Element {
    let mut result: Signal<String> = use_signal(|| String::new());

    rsx! {
        div { id: "check",
            h1 { "Check" }
            p { "Checking if the API is UP and running..." }
            button {
                onclick: move |_| {
                    spawn(async move {
                        match check_api().await {
                            Ok(resp) => {
                                *result.write() = format!(
                                    "{} (status: {})",
                                    resp.message,
                                    resp.status,
                                );
                            }
                            Err(e) => {
                                *result.write() = format!("Error: {}", e);
                            }
                        }
                    });
                },
                "Check API"
            }
            if !result().is_empty() {
                p { "{result}" }
            }
        }
    }
}

// This function makes a GET request to the /api/v1/Health_Check endpoint and returns the response or an error.
async fn check_api() -> Result<CheckResponse, String> {
    match reqwest::get("https://127.0.0.1:8080/api/v1/Health_Check").await {
        Ok(response) => match response.json::<CheckResponse>().await {
            Ok(resp) => Ok(resp),
            Err(e) => Err(format!("Failed to parse response: {}", e)),
        },
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}
