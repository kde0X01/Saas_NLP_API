use dioxus::prelude::*;

#[component]
pub fn News() -> Element {
    rsx! {
        div {
            id: "news-page",
            h1 { "News & Announcements" }
            p { "Latest headlines pulled from the backend news API." }
            section {
                class: "news-preview",
                article { h2 { "Sample Article Title" } p { "A summary of the latest announcement or news item." } }
                article { h2 { "Another Article" } p { "A second news item placeholder for the news page." } }
            }
        }
    }
}
