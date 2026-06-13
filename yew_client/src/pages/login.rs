// pages/login.rs
use yew::prelude::*;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let auth_url = "https://127.0.0.1:8080/api/v1/auth/google";

    html! {
        <div style="padding: 2rem;">
            <h2>{"Logon with Google"}</h2>
            <p>{"Click the button below to sign in using your Google account via the backend."}</p>
            <a href={auth_url}>
                <button style="padding: 10px 16px; background:#4285F4; color: white; border:none; border-radius:6px; cursor:pointer;">{"Sign in with Google"}</button>
            </a>
        </div>
    }
}
