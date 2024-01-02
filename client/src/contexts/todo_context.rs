use std::fmt;
use std::ops::Deref;

use yew::prelude::*;


/// State handle for the [`use_user_context`] hook.
pub struct UseUserContextHandle {
    todos: Vec<Todo>
}

impl UseTodosContextHandle {
    pub fn login(&self, value: UserInfo) {
        // Set global token after logged in
        set_token(Some(value.token.clone()));
        self.inner.set(value);
        // Redirect to home page
        self.navigator.push(&AppRoute::Home);
    }

    pub fn logout(&self) {
        // Clear global token after logged out
        set_token(None);
        self.inner.set(UserInfo::default());
        // Redirect to home page
        self.navigator.push(&AppRoute::Home);
    }
}

impl Deref for UseTodosContextHandle {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &self.todos
    }
}

impl Clone for UseTodosContextHandle {
    fn clone(&self) -> Self {
        Self {
            todos: self.inner.clone(),
        }
    }
}

impl PartialEq for UseTodosContextHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.todos == *other.todos
    }
}

impl fmt::Debug for UseTodosContextHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseTodosContextHandle")
            .field("value", &format!("{:?}", *self.todos))
            .finish()
    }
}

/// This hook is used to manage user context.
#[hook]
pub fn use_user_context() -> UseTodosContextHandle {
    let todos = use_context::<UseStateHandle<UserInfo>>().unwrap();
    
    UseTodosContextHandle { todos }
}
