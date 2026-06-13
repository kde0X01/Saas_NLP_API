//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//!
//! The [`Home`] and [`Blog`] components will be rendered when the current route is [`Route::Home`] or [`Route::Blog`] respectively.
//!
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes.

mod home;
pub use home::Home;

mod login;
pub use login::Login;

mod register;
pub use register::RegisterPage;

// mod dashboard_old;
// pub use dashboard_old::Dashboard;
mod dashboard;
pub use dashboard::Dashboard;

mod faq;
pub use faq::Faq;

mod offers;
pub use offers::Offers;

mod news;
pub use news::News;

// mod blog;
// pub use blog::Blog;

mod navbar;
pub use navbar::Navbar;
