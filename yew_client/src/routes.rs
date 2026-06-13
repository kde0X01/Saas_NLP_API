// routes.rs, routes for yew_router.

use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/dashboard")]
    Dashboard,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/check")]
    Check,
    #[at("/auth/callback")]
    AuthCallback,
    #[at("/faq")]
    Faq,
    #[not_found]
    #[at("/404")]
    NotFound,
}
