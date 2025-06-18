#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod components;
mod models;
mod pages;
mod utils;

use components::layout::Layout;
use pages::{Home, Login, TodoList};

// Define our application routes
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[route("/todos")]
    TodoList {},
}

fn main() {
    // Initialize console error panic hook for better debugging
    #[cfg(feature = "web")]
    console_error_panic_hook::set_once();
    
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
} 