// pages/auth_callback.rs
use crate::routes::Route;
use crate::utils;
use gloo_console::log;
use gloo_timers::future::TimeoutFuture;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Deserialize)]
struct ProtectedResponse {
    message: String,
    user: String,
}

#[function_component(AuthCallbackPage)]
pub fn auth_callback_page() -> Html {
    let message: UseStateHandle<String> = use_state(|| "Processing callback...".to_string());
    let navigator = use_navigator().unwrap();

    {
        let message: UseStateHandle<String> = message.clone();
        let navigator = navigator.clone();
        use_effect_with((), move |_| {
            // parse query params and store token if present
            if let Some(win) = window()
                && let Ok(location) = win.location().href()
            {
                // If backend redirects here with query params, try to extract token
                let href: String = location;
                let url: web_sys::Url =
                    web_sys::Url::new(&href).unwrap_or_else(|_| web_sys::Url::new("/").unwrap());
                let params = url.search_params();

                let token_opt = params.get("token").or_else(|| params.get("access_token"));

                if let Some(token) = token_opt {
                    // Store token
                    if let Ok(Some(storage)) = win.local_storage() {
                        let _ = storage.set_item("auth_token", &token);
                        message.set("Fetching user information...".to_string());

                        // Fetch user info from backend
                        let token_clone = token.clone();
                        let message_clone = message.clone();
                        let navigator_clone = navigator.clone();

                        spawn_local(async move {
                            let client = reqwest::Client::new();
                            match client
                                .get("https://127.0.0.1:8080/api/v1/protected")
                                .header("Authorization", format!("Bearer {}", token_clone))
                                .send()
                                .await
                            {
                                Ok(response) => {
                                    if response.status().is_success() {
                                        match response.json::<ProtectedResponse>().await {
                                            Ok(data) => {
                                                // Store user email
                                                utils::set_user_email(&data.user);
                                                message_clone.set(format!(
                                                    "Welcome, {}! Redirecting to dashboard...",
                                                    data.user
                                                ));

                                                // Redirect to dashboard after showing message briefly
                                                let navigator_final = navigator_clone.clone();
                                                wasm_bindgen_futures::spawn_local(async move {
                                                    // Wait 1.5 seconds before redirecting
                                                    TimeoutFuture::new(1500).await;
                                                    let _ = navigator_final.push(&Route::Dashboard);
                                                });
                                            }
                                            Err(e) => {
                                                log!(format!("Failed to parse user info: {:?}", e));
                                                message_clone.set(
                                                    "Authentication successful, but failed to get user info.".to_string()
                                                );
                                            }
                                        }
                                    } else {
                                        let status = response.status();
                                        message_clone.set(
                                            format!("Authentication successful, but failed to verify token: {}", status)
                                        );
                                    }
                                }
                                Err(e) => {
                                    log!(format!("Failed to fetch user info: {:?}", e));
                                    message_clone.set(
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
                    // no token found; maybe server set cookie; inform user
                    message.set("No token in URL. The backend may have set a cookie. Close this tab and return to the app.".to_string());
                }
            }

            // no cleanup
            || ()
        });
    }

    html! {
        <div style="padding:2rem;">
            <h2>{"OAuth Callback"}</h2>
            <p>{(*message).clone()}</p>
            <p>{"You can now close this tab or navigate back to the app."}</p>
        </div>
    }
}
