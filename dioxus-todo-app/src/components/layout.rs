use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{Route, models::{AuthState, User}, utils::{load_user, clear_user}};

#[component]
pub fn Layout() -> Element {
    // Global authentication state
    let mut auth_state = use_signal(|| {
        match load_user() {
            Some(user) => AuthState::Authenticated(user),
            None => AuthState::Unauthenticated,
        }
    });

    // Navigation function
    let navigator = use_navigator();

    // Logout handler
    let handle_logout = {
        let mut auth_state = auth_state.clone();
        move |_| {
            clear_user();
            auth_state.set(AuthState::Unauthenticated);
            navigator.push(Route::Home {});
        }
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
                            
                            match auth_state.read().clone() {
                                AuthState::Authenticated(ref user) => rsx! {
                                    Link { 
                                        to: Route::TodoList {},
                                        class: "text-gray-700 hover:text-blue-600 font-medium transition-colors",
                                        "My Todos"
                                    },
                                    span { 
                                        class: "text-gray-600",
                                        "Welcome, {user.username}!"
                                    },
                                    button { 
                                        class: "bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded-lg font-medium transition-colors",
                                        onclick: handle_logout,
                                        "Logout"
                                    }
                                },
                                AuthState::Unauthenticated => rsx! {
                                    Link { 
                                        to: Route::Login {},
                                        class: "bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg font-medium transition-colors",
                                        "Login"
                                    }
                                },
                                AuthState::Loading => rsx! {
                                    div { 
                                        class: "text-gray-500",
                                        "Loading..."
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