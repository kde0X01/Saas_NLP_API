// logout module for the webclient

use crate::components::clear_auth;
use crate::Route;
use dioxus::prelude::*;
use dioxus::router::navigator;

#[component]
pub fn Logout() -> Element {
    let nav: dioxus_router::Navigator = navigator();

    // Perform logout actions on render using the effect hook
    use_effect(move || {
        // Clear all auth data from localStorage
        clear_auth();

        // Redirect the user to the home page
        nav.replace(Route::Home {});
    });

    rsx! {
        div {
            class: "flex items-center justify-center min-h-screen",
            p { "Logging out..." }
        }
    }
}
