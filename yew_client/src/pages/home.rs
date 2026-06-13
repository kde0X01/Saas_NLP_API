// pages/home.rs
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*; // Add this import // Make sure this matches your route module

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div style="padding: 2rem;">
            // Navigation Links
            <nav style="margin-bottom: 1rem;">
                <Link<Route> to={Route::Dashboard}>{ "Go to Dashboard" }</Link<Route>>
                { " | " }
                <Link<Route> to={Route::Login}>{ "Logon" }</Link<Route>>
                { " | " }
                <Link<Route> to={Route::Register}>{ "Register" }</Link<Route>>
                { " | " }
                <Link<Route> to={Route::Check}>{ "Health Check" }</Link<Route>>
                { " | " }
                <Link<Route> to={Route::Faq}>{ "FAQ" }</Link<Route>>
            </nav>
            <h1>{ "Welcome to Our News Archive Service" }</h1>
            <p>{ "Who We Are: A team dedicated to bringing historical news access to everyone." }</p>
            <p>{ "Why We Do This: Because accessing old news should be easy and affordable." }</p>
            <p>{ "Newspapers Available: Les Echos, Le Monde, and more coming soon!" }</p>
            <p>{ "Time Range Available: From January 2020 to today." }</p>
            <p>{ "Pricing: Freemium + Premium plans." }</p>
        </div>
    }
}
