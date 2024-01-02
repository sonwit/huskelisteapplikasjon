use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
