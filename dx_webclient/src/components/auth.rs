// auth module for the webclient

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AuthState {
    pub is_authenticated: bool,
    pub user_name: Option<String>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            user_name: None,
        }
    }
}

#[component]
pub fn AuthProvider(children: Element) -> Element {
    let auth: Signal<AuthState> = use_signal(|| AuthState::default());
    use_context_provider(|| auth);

    rsx! {
        {children}
    }
}
