use yew::prelude::*;

use crate::{components::todo_item::TodoItem, types::todo::Todo};

#[derive(Properties, PartialEq)]
pub struct TodoListProps {
    pub todos: Vec<Todo>,
}

#[function_component(TodoList)]
pub fn todo_list(TodoListProps { todos }: &TodoListProps) -> Html {
    html! {
    <ul class="item-list">
       {todos
           .iter()
           .map(|todo| html!{
                <TodoItem key={todo.id} todo={todo.clone()} />
            })
           .collect::<Html>()
        }
    </ul>}
}
