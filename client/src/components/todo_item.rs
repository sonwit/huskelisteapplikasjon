use gloo_net::http::Request;
use yew::prelude::*;

use crate::types::todo::Todo;

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    pub todo: Todo,
}

#[function_component(TodoItem)]
pub fn todo_item(TodoItemProps { todo }: &TodoItemProps) -> Html {
    let item_id = format!("todo-{}", todo.id);

    
    html! {
        <li key={item_id.clone()}>
            <input
                class="vh"
                id={item_id.clone()}
                type="checkbox"
                checked={todo.completed}
                // onChange={toggle_complete}
            />
            <label for={item_id.clone()} class="text">
                <span class="tick"></span>
                <span class="text">{todo.title.clone()}</span>
            </label>
            <button
                // onClick={() => deleteTodo(todo.id, todo.title)}
                aria-label={format!("slett {}", todo.title)}
            >
            {html! {"\u{00D7}"}}
            </button>
        </li>
    }
}
