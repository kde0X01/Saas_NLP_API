// home.rs - Home page component for the news API client
// This component will be the landing page for our app. It will contain a hero section and some links to other pages in our app. It will also contain a check component that will check if the API is UP and running.
// The home page is the first page that users will see when they visit our app. It should be welcoming and informative, and it should provide links to other pages in our app.

use crate::components::Check;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            id: "home-page",
            // Hero {}

            // check API status
            Check {}
            div {
                class: "home-links",
                Link { to: Route::Login {}, "Login" }
                Link { to: Route::RegisterPage {}, "Register" }
                Link { to: Route::News {}, "News & Announcements" }
            }
        }
    }
}
