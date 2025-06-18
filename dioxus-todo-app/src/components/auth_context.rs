use dioxus::prelude::*;
use crate::models::{AuthState, User};
use crate::utils::{load_user, save_user, clear_user};

// Authentication context for sharing auth state across components
#[derive(Clone, Copy)]
pub struct AuthContext {
    pub auth_state: Signal<AuthState>,
}

impl AuthContext {
    pub fn new() -> Self {
        let auth_state = use_signal(|| {
            match load_user() {
                Some(user) => AuthState::Authenticated(user),
                None => AuthState::Unknown,
            }
        });

        Self { auth_state }
    }

    pub fn login(&mut self, user: User) -> Result<(), String> {
        save_user(&user)?;
        self.auth_state.set(AuthState::Authenticated(user));
        Ok(())
    }

    pub fn logout(&mut self) {
        clear_user();
        self.auth_state.set(AuthState::Unknown);
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(self.auth_state.read().clone(), AuthState::Authenticated(_))
    }

    pub fn current_user(&self) -> Option<User> {
        match self.auth_state.read().clone() {
            AuthState::Authenticated(user) => Some(user),
            _ => None,
        }
    }
}

// Hook for using auth context
pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>()
}

// Provider component for auth context
#[component]
pub fn AuthProvider(children: Element) -> Element {
    let auth_context = AuthContext::new();
    
    use_context_provider(|| auth_context);
    
    rsx! {
        {children}
    }
} 