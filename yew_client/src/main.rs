// main file.

use wasm_bindgen::prelude::*;
use yew::Renderer;
mod app;
mod components;
mod pages;
mod routes;
mod utils;

// main entry to the webapp.
#[wasm_bindgen(main)]
pub fn main() {
    Renderer::<crate::app::App>::new().render();
}
