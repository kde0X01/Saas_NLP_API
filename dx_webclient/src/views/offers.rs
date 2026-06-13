use dioxus::prelude::*;

#[component]
pub fn Offers() -> Element {
    rsx! {
        div {
            id: "offers-page",
            h1 { "Offers / Prices" }
            p { "Explore pricing plans and offers for the NewsAPI SaaS service." }
            ul {
                li { "Free plan: basic access to news browsing." }
                li { "Pro plan: authenticated dashboard and daily article summaries." }
                li { "Enterprise: API integration and custom coverage." }
            }
        }
    }
}
