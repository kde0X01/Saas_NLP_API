// pages/faq.rs
use yew::prelude::*;

#[function_component(FaqPage)]
pub fn faq_page() -> Html {
    html! {
        <div style="padding: 2rem;">
            <h2>{ "Frequently Asked Questions" }</h2>
            <ul>
                <li>{ "Q: Is there a free tier? A: Yes, we offer a freemium plan." }</li>
                <li>{ "Q: How often is the data updated? A: Daily." }</li>
                <li>{ "Q: Can I request other newspapers? A: Yes, contact us!" }</li>
                <li>{ "Q: Can I export the articles? A: Coming soon!" }</li>
            </ul>
        </div>
    }
}
