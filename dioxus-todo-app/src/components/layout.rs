use dioxus::prelude::*;
use dioxus_router::prelude::{Outlet, Link};
use crate::{Route, models::AuthState, utils::{load_user, clear_user}};

#[component]
pub fn Layout() -> Element {
    let mut auth_state = use_context::<Signal<AuthState>>();

    use_effect(move || {
        if matches!(*auth_state.read(), AuthState::Unknown) {
            let user = load_user();
            *auth_state.write() = match user {
                Some(u) => AuthState::Authenticated(u),
                None => AuthState::Unknown, // Stay unknown if no user
            };
        }
    });

    let handle_logout = move |_| {
        clear_user();
        *auth_state.write() = AuthState::Unknown;
    };

    rsx! {
        div { 
            class: "min-h-screen bg-gray-50",
            
            // Navigation Header
            nav { 
                class: "bg-white shadow-lg border-b border-gray-200",
                div { 
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div { 
                        class: "flex justify-between items-center h-16",
                        
                        // Logo/Brand
                        div { 
                            class: "flex items-center space-x-4",
                            Link { 
                                to: Route::Home {},
                                class: "text-2xl font-bold text-blue-600 hover:text-blue-800 transition-colors",
                                "📝 Dioxus Todo"
                            }
                        },
                        
                        // Navigation Links
                        div { 
                            class: "hidden md:flex items-center space-x-6",
                            
                            div {
                                class: "flex items-center space-x-4",
                                match &*auth_state.read() {
                                    AuthState::Authenticated(user) => rsx! {
                                        span {
                                            class: "text-white",
                                            "Welcome, {user.username}"
                                        }
                                        button {
                                            class: "px-4 py-2 text-sm text-white bg-red-600 hover:bg-red-700 rounded-md transition",
                                            onclick: handle_logout,
                                            "Logout"
                                        }
                                    },
                                    AuthState::Guest | AuthState::Unknown | AuthState::Failed => rsx! {
                                        Link {
                                            to: Route::LoginPage {},
                                            class: "px-4 py-2 text-sm text-white bg-blue-600 hover:bg-blue-700 rounded-md transition",
                                            "Login"
                                        }
                                    }
                                }
                            }
                        },
                        
                        // Mobile menu button (placeholder)
                        div { 
                            class: "md:hidden",
                            button { 
                                class: "text-gray-500 hover:text-gray-700",
                                "☰"
                            }
                        }
                    }
                }
            },
            
            // Main Content
            main { 
                class: "max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8",
                
                // Pass auth state to child components
                div { 
                    "data-auth-state": format!("{:?}", auth_state.read()),
                    Outlet::<Route> {}
                }
            },
            
            // Footer
            footer { 
                class: "bg-white border-t border-gray-200 mt-auto",
                div { 
                    class: "max-w-7xl mx-auto py-4 px-4 sm:px-6 lg:px-8 text-center text-gray-600",
                    "Built with ❤️ using Dioxus Framework"
                }
            }
        }
    }
} 