#![allow(non_snake_case)]

mod components;
mod models;
mod pages;
mod utils;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_router::components::Router;
use models::AuthState;
use wasm_bindgen::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/login")]
    LoginPage {},
    #[route("/todos")]
    TodoList {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[wasm_bindgen(start)]
pub fn main() {
    // init wasm logger
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    dioxus_web::launch(App);
}

#[component]
fn App(cx: Scope) -> Element {
    use_context_provider(|| Signal::new(AuthState::Unknown));

    rsx! {
        Router::<Route> {}
    }
} 