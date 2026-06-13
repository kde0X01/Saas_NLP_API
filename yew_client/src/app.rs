// App logic

use yew::{Children, Html, Properties, function_component, html};
use yew_router::prelude::*;

use crate::components::UserIndicator;
use crate::pages;
use crate::routes::Route;
use pages::{
    auth_callback::AuthCallbackPage, check::CheckPage, dashbord::DashboardPage, faq::FaqPage,
    home::HomePage, login::LoginPage, register::RegisterPage,
};

#[derive(Properties, PartialEq)]
struct LayoutProps {
    children: Children,
}

#[function_component(Layout)]
fn layout(props: &LayoutProps) -> Html {
    html! {
        <div>
            <UserIndicator />
            <div style="margin-top: 50px;">
                {props.children.clone()}
            </div>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> yew::Html {
    html! {
        <BrowserRouter>
            <Layout>
                <Switch<Route> render={switch} />
            </Layout>
        </BrowserRouter>
    }
}

// Switch function to render the correct page based on the route
fn switch(route: Route) -> yew::Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Login => html! { <LoginPage /> },
        Route::Register => html! { <RegisterPage /> },
        Route::Check => html! { <CheckPage /> },
        Route::AuthCallback => html! { <AuthCallbackPage /> },
        Route::Dashboard => html! { <DashboardPage /> },
        Route::Faq => html! { <FaqPage /> },
        Route::NotFound => html! { <h1>{ "404 - Not Found" }</h1> },
    }
}
