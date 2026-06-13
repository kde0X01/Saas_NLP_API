use crate::components::is_authenticated;
use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    let is_logged_in = is_authenticated();

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::News {}, "News" }
            Link { to: Route::Faq {}, "FAQ" }
            Link { to: Route::Offers {}, "Offers" }

            if !is_logged_in {
                Link { to: Route::Login {}, "Login" }
                Link { to: Route::RegisterPage {}, "Register" }
            }

            Link { to: Route::Dashboard {}, "Dashboard" }

            if is_logged_in {
                Link { to: Route::Logout {}, "Logout" }
            }
        }

        Outlet::<Route> {}
    }
}
