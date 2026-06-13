// pages/register.rs
use yew::prelude::*;

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let auth_url = "https://127.0.0.1:8080/api/v1/auth/google";

    html! {
        <div style="padding: 2rem;">
            <h2>{"Register using Google"}</h2>
            <p>{"Click to create or register an account using your Google identity."}</p>
            <a href={auth_url}>
                <button style="padding: 10px 16px; background:#34A853; color: white; border:none; border-radius:6px; cursor:pointer;">{"Register with Google"}</button>
            </a>
        </div>
    }
}
