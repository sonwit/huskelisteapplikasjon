use yew::prelude::*;

use crate::{types::todo::Todo, components::todo_item::TodoItem};


#[derive(Properties, PartialEq)]
pub struct TodoListProps {
    pub todos: Vec<Todo>,
}

#[function_component(TodoList)]
pub fn todo_list(TodoListProps { todos }: &TodoListProps) -> Html {
    todos
        .iter()
        .map(|todo| {
            html! {
                <TodoItem todo={todo.clone()} />
            }
        })
        .collect()
}
