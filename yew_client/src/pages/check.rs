// pages/check.rs
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(CheckPage)]
pub fn check_page() -> Html {
    let status_state: UseStateHandle<Option<String>> = use_state(|| None::<String>);
    let running: UseStateHandle<bool> = use_state(|| false);

    let onclick: Callback<MouseEvent> = {
        let status_state: UseStateHandle<Option<String>> = status_state.clone();
        let running: UseStateHandle<bool> = running.clone();
        Callback::from(move |_| {
            let status_state = status_state.clone();
            let running = running.clone();
            running.set(true);
            status_state.set(None);
            spawn_local(async move {
                let url = "https://127.0.0.1:8080/api/v1/Health_Check".to_string();
                match reqwest::get(&url).await {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            match resp.text().await {
                                Ok(txt) => {
                                    // try to pretty-print JSON if possible
                                    match serde_json::from_str::<Value>(&txt) {
                                        Ok(v) => status_state.set(Some(
                                            serde_json::to_string_pretty(&v).unwrap_or(txt),
                                        )),
                                        Err(_) => status_state.set(Some(txt)),
                                    }
                                }
                                Err(e) => {
                                    status_state.set(Some(format!("Failed to read body: {}", e)))
                                }
                            }
                        } else {
                            let s = resp.status();
                            let txt = resp.text().await.unwrap_or_default();
                            status_state.set(Some(format!("Error {}: {}", s, txt)));
                        }
                    }
                    Err(e) => status_state.set(Some(format!("Network error: {}", e))),
                }
                running.set(false);
            });
        })
    };

    html! {
        <div style="padding: 2rem;">
            <h2>{"Backend Health Check"}</h2>
            <p>{"This will call the backend Health_Check endpoint and display the response."}</p>
            <button {onclick} disabled ={*running} style="padding: 8px 12px;">{ if *running { "Checking..." } else { "Check API" } }</button>
            {
                if let Some(result) = &*status_state {
                    html!{
                        <pre style="margin-top:1rem; background:#f6f6f6; padding:10px; border-radius:6px;">{ result.clone() }</pre>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
