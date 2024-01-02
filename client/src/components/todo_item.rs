use yew::prelude::*;

use crate::types::todo::Todo;

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    pub todo: Todo,
}

#[function_component(TodoItem)]
pub fn todo_item(TodoItemProps { todo }: &TodoItemProps) -> Html {
    html! {<p key={todo.id}>{format!("{}: {}", todo.title, todo.completed)}</p>}
}
