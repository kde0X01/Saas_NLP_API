// Dashboard after authentication is successful
// Main logic for a Dioxus client that fetches data from a Rust API

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

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
#[component]
pub fn Dashboard() -> Element {
    // State for the input fields, matching the API query parameters.
    // Defaulting to a date that previously worked to make testing easier.
    let mut year_state: Signal<String> = use_signal(|| "2021".to_string());
    let mut month_state: Signal<String> = use_signal(|| "1".to_string());
    let mut day_state: Signal<String> = use_signal(|| "15".to_string());

    let mut data_state: Signal<Option<FinalData>> = use_signal(|| None);
    let mut error_state: Signal<Option<String>> = use_signal(|| None);
    let mut loading_state: Signal<bool> = use_signal(|| false);

    // Callback for the button click to fetch data
    let fetch_data = move |_| {
        let year: String = year_state.read().clone();
        let month: String = month_state.read().clone();
        let day: String = day_state.read().clone();

        loading_state.set(true);
        error_state.set(None); // Clear previous errors
        data_state.set(None); // Clear previous data

        // Start the async request to fetch data from the API
        spawn(async move {
            // Construct the URL with query parameters
            let url = format!(
                "https://127.0.0.1:8080/api/v1/articles?year={}&month={}&day={}",
                year, month, day
            );

            // Get token from localStorage
            // NOTE: In Dioxus, use web_sys or a utility function to access localStorage.
            // Replace `crate::utils::get_token()` with your actual token-retrieval logic.
            let token: Option<String> = crate::components::get_token();

            // Create HTTP client
            let client: reqwest::Client = reqwest::Client::new();
            let mut request = client.get(&url);

            // Add Authorization header if token exists
            if let Some(token) = token {
                request = request.header("Authorization", format!("Bearer {}", token));
            }

            match request.send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        // Read the response body as text first for debugging
                        match response.text().await {
                            Ok(response_text) => {
                                // Try to parse the text as JSON
                                match serde_json::from_str::<FinalData>(&response_text) {
                                    Ok(data) => {
                                        data_state.set(Some(data));
                                    }
                                    Err(e) => {
                                        let body_snippet: String =
                                            response_text.chars().take(200).collect::<String>();
                                        error_state.set(Some(format!(
                                            "Failed to parse data: {}. Response snippet: '{}...'",
                                            e, body_snippet
                                        )));
                                    }
                                }
                            }
                            Err(e) => {
                                error_state
                                    .set(Some(format!("Failed to read response body: {}", e)));
                            }
                        }
                    } else {
                        let status: reqwest::StatusCode = response.status();
                        let text: String = response
                            .text()
                            .await
                            .unwrap_or_else(|_| "Could not read error response body".to_string());
                        error_state.set(Some(format!("API Error: {} - {}", status, text)));
                    }
                }
                Err(e) => {
                    error_state.set(Some(format!("Network error: {}", e)));
                }
            }

            // Regardless of success or failure, set loading to false
            loading_state.set(false);
        });
    };

    rsx! {
        div {
            style: "font-family: sans-serif; max-width: 800px; margin: 20px auto; padding: 20px; border: 1px solid #ccc; border-radius: 8px;",

            h1 { "Rust API Client (Dioxus)" }

            div {
                style: "margin-bottom: 20px; display: flex; gap: 15px; align-items: flex-end;",

                div {
                    style: "flex-grow: 1;",
                    label {
                        r#for: "year_input",
                        style: "display: block; margin-bottom: 5px;",
                        "Year:"
                    }
                    input {
                        id: "year_input",
                        r#type: "number",
                        value: "{year_state}",
                        oninput: move |e| year_state.set(e.value()),
                        placeholder: "2024",
                        style: "width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                    }
                }

                div {
                    style: "flex-grow: 1;",
                    label {
                        r#for: "month_input",
                        style: "display: block; margin-bottom: 5px;",
                        "Month:"
                    }
                    input {
                        id: "month_input",
                        r#type: "number",
                        value: "{month_state}",
                        oninput: move |e| month_state.set(e.value()),
                        placeholder: "5",
                        style: "width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                    }
                }

                div {
                    style: "flex-grow: 1;",
                    label {
                        r#for: "day_input",
                        style: "display: block; margin-bottom: 5px;",
                        "Day:"
                    }
                    input {
                        id: "day_input",
                        r#type: "number",
                        value: "{day_state}",
                        oninput: move |e| day_state.set(e.value()),
                        placeholder: "21",
                        style: "width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                    }
                }
            }

            button {
                onclick: fetch_data,
                disabled: *loading_state.read(),
                style: "padding: 10px 20px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                if *loading_state.read() { "Loading..." } else { "Fetch Data" }
            }

            // Display error message if any
            if let Some(error) = error_state.read().as_ref() {
                p {
                    style: "color: red; margin-top: 20px;",
                    strong { "Error: " }
                    { error.clone() }
                }
            }

            // Display fetched data if available
            if let Some(data) = data_state.read().as_ref() {
                if data.data_results.is_empty() {
                    p {
                        style: "margin-top: 20px; color: #555;",
                        "No articles found for the selected table and date."
                    }
                } else {
                    div {
                        style: "margin-top: 20px;",
                        h2 { "Articles ({data.total_results} found):" }

                        table {
                            style: "width: 100%; border-collapse: collapse; margin-top: 10px;",
                            thead {
                                tr {
                                    style: "background-color: #f2f2f2;",
                                    th { "Date" }
                                    th { "Category" }
                                    th { "Title" }
                                    th { "Link" }
                                }
                            }
                            tbody {
                                for article in data.data_results.iter() {
                                    tr {
                                        td { { article.pub_date() } }
                                        td { { article.category.clone() } }
                                        td { { article.title.clone() } }
                                        td {
                                            a {
                                                href: "{article.link}",
                                                target: "_blank",
                                                "Link"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
