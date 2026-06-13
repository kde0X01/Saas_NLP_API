use dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Login() -> Element {
    let auth_url = "https://127.0.0.1:8080/api/v1/auth/google";

    rsx! {
        div { style: "padding: 2rem;",
            h2 { "Logon with Google" }
            p { "Click the button below to sign in using your Google account via the backend." }
            a { href: auth_url,
                button { style: "padding: 10px 16px; background:#4285F4; color: white; border:none; border-radius:6px; cursor:pointer;",
                    "Sign in with Google"
                }
            }
        }
    }
}
