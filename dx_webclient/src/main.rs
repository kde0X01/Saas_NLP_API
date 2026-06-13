// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use components::{AuthCallbackPage, AuthProvider, Logout};
use views::{Dashboard, Faq, Home, Login, Navbar, News, Offers, RegisterPage};

/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
///
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/login")]
        Login {},
        #[route("/auth/callback")]
        AuthCallbackPage {},
        #[route("/register")]
        RegisterPage {},
        #[route("/dashboard")]
        Dashboard {},
        #[route("/faq")]
        Faq {},
        #[route("/offers")]
        Offers {},
        #[route("/news")]
        News {},
        #[route("/logout")]
        Logout {},
        // #[route("/blog/:id")]
        // Blog { id: i32 },
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete

#[allow(non_snake_case)]
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // The AuthProvider component will provide the auth context to all components in our app.
        // It should wrap the entire app so that any component can access the auth context.
        AuthProvider { Router::<Route> {} }
    }
}
