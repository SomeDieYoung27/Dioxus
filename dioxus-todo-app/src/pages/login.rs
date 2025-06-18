use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use dioxus_router::prelude::Link;
use crate::{Route, models::LoginForm, utils::authenticate_user};
use crate::{
    models::{Credentials, AuthState},
    utils::login_user,
};

#[component]
pub fn LoginPage() -> Element {
    let mut form = use_signal(LoginForm::default);
    let error_message = use_signal(|| None::<String>);
    let is_submitting = use_signal(|| false);
    let mut creds = use_signal(Credentials::default);
    let navigator = use_navigator();
    let mut auth_state = use_context::<Signal<AuthState>>();

    // Form handlers
    let handle_username_change = {
        let mut form = form.clone();
        move |evt: FormEvent| {
            form.with_mut(|f| f.username = evt.value());
        }
    };

    let handle_password_change = {
        let mut form = form.clone();
        move |evt: FormEvent| {
            form.with_mut(|f| f.password = evt.value());
        }
    };

    let handle_login = move |_evt: FormEvent| {
        let form_data = form.write();
        creds.write().username = form_data.username.clone();
        creds.write().password = form_data.password.clone();
        spawn({
            let creds = creds.read().clone();
            async move {
                match login_user(creds).await {
                    Ok(user) => {
                        *auth_state.write() = AuthState::Authenticated(user);
                        navigator.push(Route::TodoList {});
                    }
                    Err(e) => {
                        log::error!("Login failed: {}", e);
                        *auth_state.write() = AuthState::Failed;
                    }
                }
            }
        });
    };

    rsx! {
        div { 
            class: "min-h-screen bg-gray-50 flex flex-col justify-center py-12 sm:px-6 lg:px-8",
            
            div { 
                class: "sm:mx-auto sm:w-full sm:max-w-md",
                
                // Header
                div { 
                    class: "text-center",
                    h2 { 
                        class: "text-3xl font-bold text-gray-900 mb-2",
                        "Welcome Back!"
                    },
                    p { 
                        class: "text-gray-600 mb-8",
                        "Sign in to access your todos"
                    }
                }
            },
            
            div { 
                class: "sm:mx-auto sm:w-full sm:max-w-md",
                div { 
                    class: "bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10",
                    
                    // Login form
                    form { 
                        class: "space-y-6",
                        onsubmit: handle_login,
                        
                        // Username field
                        div {
                            label { 
                                class: "block text-sm font-medium text-gray-700",
                                r#for: "username",
                                "Username"
                            },
                            div { 
                                class: "mt-1",
                                input { 
                                    r#type: "text",
                                    id: "username",
                                    class: "appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md placeholder-gray-400 focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                    placeholder: "Enter your username",
                                    value: "{form.read().username}",
                                    oninput: handle_username_change,
                                    required: true
                                }
                            }
                        },
                        
                        // Password field
                        div {
                            label { 
                                class: "block text-sm font-medium text-gray-700",
                                r#for: "password",
                                "Password"
                            },
                            div { 
                                class: "mt-1",
                                input { 
                                    r#type: "password",
                                    id: "password",
                                    class: "appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md placeholder-gray-400 focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                                    placeholder: "Enter your password",
                                    value: "{form.read().password}",
                                    oninput: handle_password_change,
                                    required: true
                                }
                            }
                        },
                        
                        // Error message
                        if let Some(ref error) = error_message.read().clone() {
                            div { 
                                class: "bg-red-50 border border-red-200 text-red-600 px-4 py-3 rounded-md text-sm",
                                "❌ {error}"
                            }
                        },
                        
                        // Submit button
                        div {
                            button { 
                                r#type: "submit",
                                class: "w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed",
                                disabled: is_submitting.read().clone(),
                                
                                if *is_submitting.read() {
                                    "Signing in..."
                                } else {
                                    "Sign In"
                                }
                            }
                        }
                    },
                    
                    // Demo instructions
                    div { 
                        class: "mt-6 border-t border-gray-200 pt-6",
                        div { 
                            class: "bg-blue-50 border border-blue-200 rounded-md p-4",
                            h4 { 
                                class: "text-sm font-medium text-blue-800 mb-2",
                                "🔍 Demo Instructions"
                            },
                            div { 
                                class: "text-sm text-blue-700 space-y-1",
                                p { "• Username: minimum 3 characters" },
                                p { "• Password: minimum 6 characters" },
                                p { "• Try: username=\"demo\", password=\"password123\"" }
                            }
                        }
                    },
                    
                    // Navigation
                    div { 
                        class: "mt-6 text-center",
                        Link { 
                            to: Route::Home {},
                            class: "text-blue-600 hover:text-blue-800 text-sm font-medium",
                            "← Back to Home"
                        }
                    }
                }
            },
            
            // Framework info
            div { 
                class: "mt-8 text-center text-sm text-gray-500",
                "Built with "
                span { 
                    class: "text-blue-600 font-medium",
                    "Dioxus"
                }
                " - A React-like framework for Rust"
            }
        }
    }
} 