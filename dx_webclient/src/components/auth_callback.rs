// Authentication Callback Handler

// pages/auth_callback.rs
use crate::components::utils;
use crate::Route;
use dioxus::prelude::*;
use serde::Deserialize;
use web_sys::window;

#[derive(Deserialize)]
struct ProtectedResponse {
    message: String,
    user: String,
}

#[allow(non_snake_case)]
#[component]
pub fn AuthCallbackPage() -> Element {
    // use_signal replaces Yew's use_state
    let mut message = use_signal(|| "Processing callback...".to_string());

    // use_navigator() works the same in Dioxus router
    let navigator = use_navigator();

    // use_effect runs once on mount (no dependency list needed; just don't read any signals
    // inside it to avoid re-runs, mirroring Yew's use_effect_with((), ...))
    use_effect(move || {
        if let Some(win) = window() {
            if let Ok(href) = win.location().href() {
                // Parse the URL and pull out the token query param
                let url = match web_sys::Url::new(&href) {
                    Ok(u) => u,
                    Err(_) => return,
                };
                let params = url.search_params();
                let token_opt = params.get("token").or_else(|| params.get("access_token"));

                if let Some(token) = token_opt {
                    // Store token in localStorage
                    if let Ok(Some(storage)) = win.local_storage() {
                        let _ = storage.set_item("auth_token", &token);
                        message.set("Fetching user information...".to_string());

                        // Dioxus uses spawn() instead of wasm_bindgen_futures::spawn_local()
                        spawn(async move {
                            let client = reqwest::Client::new();
                            match client
                                .get("https://127.0.0.1:8080/api/v1/protected")
                                .header("Authorization", format!("Bearer {}", token))
                                .send()
                                .await
                            {
                                Ok(response) => {
                                    if response.status().is_success() {
                                        match response.json::<ProtectedResponse>().await {
                                            Ok(data) => {
                                                utils::set_user_email(&data.user);
                                                message.set(format!(
                                                    "Welcome, {}! Redirecting to dashboard...",
                                                    data.user
                                                ));

                                                // Wait 1.5 s then navigate — TimeoutFuture is
                                                // replaced by gloo_timers or a simple async sleep.
                                                // gloo_timers still works fine in Dioxus/WASM:
                                                gloo_timers::future::TimeoutFuture::new(1500).await;
                                                navigator.push(Route::Dashboard {});
                                            }
                                            Err(e) => {
                                                gloo_console::error!(&format!(
                                                    "Failed to parse user info: {e}"
                                                ));
                                                message.set(
                                                    "Authentication successful, but failed to get user info."
                                                        .to_string(),
                                                );
                                            }
                                        }
                                    } else {
                                        let status = response.status();
                                        message.set(format!(
                                            "Authentication successful, but failed to verify token: {}",
                                            status
                                        ));
                                    }
                                }
                                Err(e) => {
                                    gloo_console::error!(&format!(
                                        "Failed to fetch user info: {e}"
                                    ));
                                    message.set(
                                        "Authentication successful, but failed to fetch user info."
                                            .to_string(),
                                    );
                                }
                            }
                        });
                    } else {
                        message.set(
                            "Authentication successful, but localStorage unavailable.".to_string(),
                        );
                    }
                } else {
                    message.set(
                        "No token in URL. The backend may have set a cookie. \
                         Close this tab and return to the app."
                            .to_string(),
                    );
                }
            }
        }
    });

    // rsx! replaces Yew's html! macro; syntax is almost identical
    rsx! {
        div { style: "padding:2rem;",
            h2 { "OAuth Callback" }
            p { "{message}" }
            p { "You can now close this tab or navigate back to the app." }
        }
    }
}
