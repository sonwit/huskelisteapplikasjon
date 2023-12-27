use axum::{routing::get, Router};
mod features {
    pub mod todo;
}
use features::todo::{api_delete_todo, api_get_todo, api_get_todos, api_post_todo, api_put_todo};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route(
            "/api/todo",
            get(api_get_todo)
                .post(api_post_todo)
                .put(api_put_todo)
                .delete(api_delete_todo)
        )
        .route("/api/todos", get(api_get_todos));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
