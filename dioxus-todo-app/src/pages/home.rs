use crate::{
    components::Layout,
    models::AuthState,
    utils::load_user,
    Route,
};
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;

#[component]
pub fn Home() -> Element {
    let mut auth_state = use_context::<Signal<AuthState>>();
    let navigator = use_navigator();

    use_effect(move || {
        if matches!(*auth_state.read(), AuthState::Unknown) {
            if let Some(user) = load_user() {
                *auth_state.write() = AuthState::Authenticated(user);
            } else {
                *auth_state.write() = AuthState::Guest;
            }
        }
    });

    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100",
            
            // Hero Section
            div { 
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
                
                div { 
                    class: "text-center",
                    
                    // Hero Title
                    h1 { 
                        class: "text-4xl md:text-6xl font-bold text-gray-900 mb-6",
                        "Welcome to "
                        span { 
                            class: "text-blue-600",
                            "Dioxus Todo"
                        }
                    },
                    
                    // Hero Subtitle
                    p { 
                        class: "text-xl md:text-2xl text-gray-600 mb-8 max-w-3xl mx-auto",
                        "A modern, full-featured todo application built with the Dioxus framework. "
                        "Experience the power of Rust in web development!"
                    },
                    
                    // Call to Action
                    div { 
                        class: "flex flex-col sm:flex-row gap-4 justify-center items-center mb-12",
                        
                        button { 
                            class: "bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-8 rounded-lg text-lg transition-colors shadow-lg hover:shadow-xl",
                            onclick: move |_| {
                                navigator.push(Route::TodoList {});
                            },
                            "Get Started"
                        },
                        
                        a {
                            href: "https://dioxuslabs.com/learn/0.6/guide/",
                            class: "text-blue-600 hover:text-blue-800 font-semibold text-lg underline transition-colors",
                            "Learn about Dioxus →"
                        }
                    }
                }
            },
            
            // Features Section
            div { 
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
                
                h2 { 
                    class: "text-3xl font-bold text-center text-gray-900 mb-12",
                    "Why Choose Dioxus?"
                },
                
                div { 
                    class: "grid md:grid-cols-3 gap-8",
                    
                    // Feature 1
                    div { 
                        class: "bg-white p-6 rounded-xl shadow-md hover:shadow-lg transition-shadow",
                        div { 
                            class: "text-4xl mb-4",
                            "🚀"
                        },
                        h3 { 
                            class: "text-xl font-semibold text-gray-900 mb-2",
                            "Cross-Platform"
                        },
                        p { 
                            class: "text-gray-600",
                            "Write once, run everywhere. Build web, desktop, and mobile apps with the same codebase."
                        }
                    },
                    
                    // Feature 2
                    div { 
                        class: "bg-white p-6 rounded-xl shadow-md hover:shadow-lg transition-shadow",
                        div { 
                            class: "text-4xl mb-4",
                            "⚡"
                        },
                        h3 { 
                            class: "text-xl font-semibold text-gray-900 mb-2",
                            "Blazing Fast"
                        },
                        p { 
                            class: "text-gray-600",
                            "Powered by Rust's performance and memory safety. Zero-cost abstractions and minimal runtime overhead."
                        }
                    },
                    
                    // Feature 3
                    div { 
                        class: "bg-white p-6 rounded-xl shadow-md hover:shadow-lg transition-shadow",
                        div { 
                            class: "text-4xl mb-4",
                            "🛡️"
                        },
                        h3 { 
                            class: "text-xl font-semibold text-gray-900 mb-2",
                            "Type Safe"
                        },
                        p { 
                            class: "text-gray-600",
                            "Catch errors at compile time. Rust's type system ensures your apps are reliable and maintainable."
                        }
                    }
                }
            },
            
            // Tech Stack Section
            div { 
                class: "bg-white py-16",
                div { 
                    class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    
                    h2 { 
                        class: "text-3xl font-bold text-center text-gray-900 mb-12",
                        "This App Demonstrates"
                    },
                    
                    div { 
                        class: "grid md:grid-cols-2 lg:grid-cols-4 gap-6",
                        
                        // Demo features
                        for feature in [
                            ("🎨", "RSX Syntax", "React-like component syntax"),
                            ("🔄", "State Management", "Signals and reactive updates"),
                            ("🛣️", "Routing", "Client-side navigation"),
                            ("📱", "Responsive Design", "Mobile-first approach"),
                            ("🔐", "Authentication", "User login and sessions"),
                            ("💾", "Local Storage", "Data persistence"),
                            ("🎯", "Event Handling", "Interactive user interfaces"),
                            ("🧩", "Component Props", "Reusable components")
                        ] {
                            div { 
                                class: "text-center p-4 border border-gray-200 rounded-lg hover:border-blue-300 transition-colors",
                                div { 
                                    class: "text-2xl mb-2",
                                    "{feature.0}"
                                },
                                h4 { 
                                    class: "font-semibold text-gray-900 mb-1",
                                    "{feature.1}"
                                },
                                p { 
                                    class: "text-sm text-gray-600",
                                    "{feature.2}"
                                }
                            }
                        }
                    }
                }
            },
            
            // Getting Started Section
            div { 
                class: "bg-gray-50 py-16",
                div { 
                    class: "max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center",
                    
                    h2 { 
                        class: "text-3xl font-bold text-gray-900 mb-6",
                        "Ready to Start Building?"
                    },
                    
                    p { 
                        class: "text-lg text-gray-600 mb-8",
                        "This todo app showcases all the core concepts you need to build modern applications with Dioxus. "
                        "Perfect for learning and interview preparation!"
                    },
                    
                    div { 
                        class: "flex flex-col sm:flex-row gap-4 justify-center",
                        
                        button { 
                            class: "bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-6 rounded-lg transition-colors",
                            onclick: move |_| {
                                navigator.push(Route::TodoList {});
                            },
                            "Try the App"
                        },
                        
                        a {
                            href: "https://github.com/DioxusLabs/dioxus",
                            class: "bg-gray-800 hover:bg-gray-900 text-white font-semibold py-3 px-6 rounded-lg transition-colors",
                            "View Source on GitHub"
                        }
                    }
                }
            }
        }
    }
} 