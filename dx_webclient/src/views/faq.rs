use dioxus::prelude::*;

#[component]
pub fn Faq() -> Element {
    rsx! {
        div {
            id: "faq-page",
            h1 { "FAQ / Help" }
            p { "Find answers to common questions about the NewsAPI portal." }
            ul {
                li { "How do I authenticate? Use the Login page or the Google auth flow." }
                li { "Where can I see news? Visit the News page or the Dashboard once logged in." }
                li { "How is data loaded? The app loads articles from the backend API by date." }
            }
        }
    }
}
