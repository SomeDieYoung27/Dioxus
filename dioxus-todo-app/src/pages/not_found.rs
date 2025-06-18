use dioxus::prelude::*;
use dioxus_router::prelude::Link;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "text-center py-20",
            h1 {
                class: "text-4xl font-bold text-red-600",
                "Page Not Found"
            }
            p {
                class: "mt-4 text-lg text-gray-700",
                "The page you're looking for does not exist."
            }
            p {
                class: "mt-2 text-sm text-gray-500",
                code { "Route: /{route.join(\"/\")}" }
            }
            div {
                class: "mt-8",
                Link {
                    to: crate::Route::Home {},
                    class: "px-6 py-3 bg-blue-600 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 transition-colors",
                    "Go back home"
                }
            }
        }
    }
} 