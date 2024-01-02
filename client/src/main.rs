use client::{types::todo::Todo, components::todo_list::TodoList};
use yew::prelude::*;
use gloo_net::http::Request;

#[function_component(App)]
fn app() -> Html {

    let todos = use_state(|| vec![]);
    {
       let todos = todos.clone();
       use_effect_with((), move |_| {
           let todos = todos.clone();
           wasm_bindgen_futures::spawn_local(async move {
               let fetched_todos: Vec<Todo> = Request::get("http://localhost:3000/api/todos")
                   .send()
                   .await
                   .unwrap()
                   .json()
                   .await
                   .unwrap();
               todos.set(fetched_todos);
           });
           || ()
       });
   }

    html! {
        <div class="App">
            <h1>{"Huskeliste"}</h1>
            <TodoList todos={(*todos).clone()} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
