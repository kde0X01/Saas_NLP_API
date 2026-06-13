// components.rs - Reusable components

use crate::routes::Route;
use crate::utils;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(UserIndicator)]
pub fn user_indicator() -> Html {
    let user_email = use_state(|| utils::get_user_email());
    let is_authenticated = utils::is_authenticated();

    // Refresh user email on mount
    {
        let user_email = user_email.clone();
        use_effect_with((), move |_| {
            user_email.set(utils::get_user_email());
            || ()
        });
    }

    html! {
        <div style="position: fixed; top: 0; right: 0; padding: 10px 20px; background-color: #f0f0f0; border-bottom-left-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); z-index: 1000;">
            {
                if is_authenticated {
                    if let Some(email) = &*user_email {
                        html! {
                            <div style="display: flex; align-items: center; gap: 10px;">
                                <span style="color: #333; font-weight: 500;">
                                    {"Logged in as: "}
                                    <span style="color: #007bff;">{email}</span>
                                </span>
                                <span style="margin-left: 10px;">
                                    <Link<Route> 
                                        to={Route::Dashboard}
                                        classes="btn-link"
                                    >
                                        <span style="color: #007bff; text-decoration: none;">{"Dashboard"}</span>
                                    </Link<Route>>
                                </span>
                            </div>
                        }
                    } else {
                        html! {
                            <div>
                                <span style="color: #333;">{"Authenticated"}</span>
                                <span style="margin-left: 10px;">
                                    <Link<Route> 
                                        to={Route::Dashboard}
                                        classes="btn-link"
                                    >
                                        <span style="color: #007bff; text-decoration: none;">{"Dashboard"}</span>
                                    </Link<Route>>
                                </span>
                            </div>
                        }
                    }
                } else {
                    html! {
                        <div>
                            <Link<Route> 
                                to={Route::Login}
                                classes="btn-link"
                            >
                                <span style="color: #007bff; text-decoration: none;">{"Login"}</span>
                            </Link<Route>>
                        </div>
                    }
                }
            }
        </div>
    }
}

