// Dashborad after the authentication is success
// main logic for a Yew client that fetches data from a Rust API

use gloo_console::log;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
// use web_sys::HtmlInputElement;
use yew::prelude::*;

// now the data structure for the dashboard
// Data structure mirroring the API response
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Article {
    pub pub_year: u32,
    pub pub_month: u32,
    pub pub_day: u32,
    pub category: String,
    pub title: String,
    pub link: String,
    pub content: String,
}

// Helper method to format the publication date
impl Article {
    pub fn pub_date(&self) -> String {
        format!("{}-{}-{}", self.pub_year, self.pub_month, self.pub_day)
    }
}

// Structure for the final data received from the API
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct FinalData {
    total_results: usize,
    data_results: Vec<Article>,
}

// Dashboard component
#[function_component(DashboardPage)]
pub fn dashboard() -> Html {
    log!("Yew client started");
    // State for the input fields, matching the new API query parameters.
    // Defaulting to a date that previously worked to make testing easier.
    let year_state: UseStateHandle<String> = use_state(|| "2021".to_string());
    let month_state: UseStateHandle<String> = use_state(|| "1".to_string());
    let day_state: UseStateHandle<String> = use_state(|| "15".to_string());

    let data_state: UseStateHandle<Option<FinalData>> = use_state(|| None); // State to hold fetched data
    let error_state: UseStateHandle<Option<String>> = use_state(|| None); // State to hold error messages
    let loading_state: UseStateHandle<bool> = use_state(|| false); // State to indicate loading status

    // --- Callbacks for input changes ---
    let on_year_change = {
        let year_state: UseStateHandle<String> = year_state.clone();
        move |e: Event| {
            let input: web_sys::HtmlInputElement =
                e.target_unchecked_into::<web_sys::HtmlInputElement>();
            year_state.set(input.value());
        }
    };

    // Callback for month input change
    let on_month_change = {
        let month_state: UseStateHandle<String> = month_state.clone();
        move |e: Event| {
            let input: web_sys::HtmlInputElement =
                e.target_unchecked_into::<web_sys::HtmlInputElement>();
            month_state.set(input.value());
        }
    };

    // Callback for day input change
    let on_day_change = {
        let day_state: UseStateHandle<String> = day_state.clone();
        move |e: Event| {
            let input: web_sys::HtmlInputElement =
                e.target_unchecked_into::<web_sys::HtmlInputElement>();
            day_state.set(input.value());
        }
    };

    // Callback for the button click to fetch data
    let onclick: Callback<MouseEvent> = Callback::from({
        let year: String = (*year_state).clone();
        let month: String = (*month_state).clone();
        let day: String = (*day_state).clone();
        let data_state: UseStateHandle<Option<FinalData>> = data_state.clone();
        let error_state: UseStateHandle<Option<String>> = error_state.clone();
        let loading_state: UseStateHandle<bool> = loading_state.clone();

        move |_| {
            let year: String = year.clone();
            let month: String = month.clone();
            let day: String = day.clone();
            let data_state: UseStateHandle<Option<FinalData>> = data_state.clone();
            let error_state: UseStateHandle<Option<String>> = error_state.clone();
            let loading_state: UseStateHandle<bool> = loading_state.clone();

            loading_state.set(true);
            error_state.set(None); // Clear previous errors
            data_state.set(None); // Clear previous data

            // Start the async request to fetch data from the API
            spawn_local(async move {
                // Construct the new URL with query parameters
                let url: String = format!(
                    //
                    "https://127.0.0.1:8080/api/v1/articles?year={}&month={}&day={}",
                    year, month, day
                );

                log!(format!("Fetching from: {}", url));

                // Get token from localStorage
                let token = crate::utils::get_token();

                // Create HTTP client
                let client = reqwest::Client::new();
                let mut request = client.get(&url);

                // Add Authorization header if token exists
                if let Some(token) = token {
                    request = request.header("Authorization", format!("Bearer {}", token));
                    log!("Added Authorization header with token");
                } else {
                    log!("No token found, request will fail authentication");
                }

                match request.send().await {
                    Ok(response) => {
                        if response.status().is_success() {
                            // Read the response body as text first for debugging
                            let response_text: String = match response.text().await {
                                Ok(text) => text,
                                Err(e) => {
                                    log!(format!("Failed to read response body as text: {:?}", e));
                                    error_state
                                        .set(Some(format!("Failed to read response body: {}", e)));
                                    loading_state.set(false);
                                    return; // Exit spawn_local block
                                }
                            };
                            log!(format!("Raw response body: {}", response_text));

                            // Now try to parse the text as JSON
                            match serde_json::from_str::<FinalData>(&response_text) {
                                Ok(data) => {
                                    log!(format!("Data received: {:?}", data));
                                    data_state.set(Some(data));
                                }
                                // If parsing fails, log the error and set the error state
                                Err(e) => {
                                    log!(format!("Failed to parse JSON from text: {:?}", e));
                                    // Include a snippet of the body in the error for easier debugging
                                    let body_snippet =
                                        response_text.chars().take(200).collect::<String>();
                                    error_state.set(Some(format!(
                                        "Failed to parse data: {}. Response snippet: '{}...'",
                                        e, body_snippet
                                    )));
                                }
                            }
                            // If parsing is successful, the data_state will be updated
                        } else {
                            let status: reqwest::StatusCode = response.status();
                            let text: String = response.text().await.unwrap_or_else(|_| {
                                "Could not read error response body".to_string()
                            });
                            log!(format!("API error: Status {} - {}", status, text));
                            error_state.set(Some(format!("API Error: {} - {}", status, text)));
                        }
                    }
                    // If the request fails, log the error and set the error state
                    Err(e) => {
                        log!(format!("Request failed: {:?}", e));
                        error_state.set(Some(format!("Network error: {}", e)));
                    }
                }
                // Regardless of success or failure, set loading to false
                loading_state.set(false);
            });
        }
    });

    // Render the HTML structure of the app
    html! {
        <div style="font-family: sans-serif; max-width: 800px; margin: 20px auto; padding: 20px; border: 1px solid #ccc; border-radius: 8px;">
            <h1>{"Rust API Client (Yew)"}</h1>

            <div style="margin-bottom: 20px; display: flex; gap: 15px; align-items: flex-end;">
                <div style="flex-grow: 1;">
                    <label for="year_input" style="display: block; margin-bottom: 5px;">{"Year:"}</label>
                    <input
                        id="year_input"
                        type="number"
                        value={(*year_state).clone()}
                        onchange={Callback::from(on_year_change)}
                        placeholder="2024"
                        style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>
                <div style="flex-grow: 1;">
                    <label for="month_input" style="display: block; margin-bottom: 5px;">{"Month:"}</label>
                    <input
                        id="month_input"
                        type="number"
                        value={(*month_state).clone()}
                        onchange={Callback::from(on_month_change)}
                        placeholder="5"
                        style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>
                <div style="flex-grow: 1;">
                    <label for="day_input" style="display: block; margin-bottom: 5px;">{"Day:"}</label>
                    <input
                        id="day_input"
                        type="number"
                        value={(*day_state).clone()}
                        onchange={Callback::from(on_day_change)}
                        placeholder="21"
                        style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>
            </div>

            <button
                onclick={onclick}
                disabled={*loading_state}
                style="padding: 10px 20px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;"
            >
                { if *loading_state { "Loading..." } else { "Fetch Data" } }
            </button>

            {
                // Display error message if any
                if let Some(error) = &*error_state {
                    html! {
                        <p style="color: red; margin-top: 20px;">
                            <strong>{"Error:"}</strong> {error}
                        </p>
                    }
                // Display loading message
                } else {
                    html! {}
                }
            }

            // Display fetched data if available
            {
                if let Some(data) = &*data_state {
                    if data.data_results.is_empty() {
                        html! {
                            <p style="margin-top: 20px; color: #555;">{"No articles found for the selected table and date."}</p>
                        }
                    } else {
                        html! {
                            <div style="margin-top: 20px;">
                                <h2>{format!("Articles ({} found):", data.total_results)}</h2>

                                <table style="width: 100%; border-collapse: collapse; margin-top: 10px;" border_spacing=30px>
                                    <thead>
                                        <tr style="background-color: #f2f2f2;">
                                            // <th>{"Year"}</th>
                                            // <th>{"Month"}</th>
                                            // <th>{"Day"}</th>
                                            <th> {"Date"}</th>
                                            <th>{"Category"}</th>
                                            <th>{"Title"}</th>
                                            <th>{"Link"}</th>
                                            // <th>{"Content"}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        { for data.data_results.iter().map(|article| html! {
                                            <tr>
                                                // <td>{article.pub_year}</td>
                                                // <td>{article.pub_month}</td>
                                                // <td>{article.pub_day}</td>
                                                <td>{article.pub_date()}</td>
                                                <td>{article.category.clone()}</td>
                                                <td>{article.title.clone()}</td>
                                                <td>
                                                    <a href={article.link.clone()} target="_blank">{ "Link" }</a>
                                                </td>
                                                // <td>{article.content.clone()}</td>
                                            </tr>
                                        }) }
                                    </tbody>
                                </table>
                            </div>
                        }
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
