use axum::{
    http::{self, HeaderValue},
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;
mod features {
    pub mod todo;
}
use axum::http::Method;
use features::todo::{api_delete_todo, api_get_todo, api_get_todos, api_post_todo, api_put_todo};

#[tokio::main]
async fn main() {
    let allowed_origins = vec![
        HeaderValue::from_static("http://localhost:3006"),
        HeaderValue::from_static("http://127.0.0.1:3006"),
    ];

    let cors_layer = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(allowed_origins)
        .allow_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .allow_credentials(true);

    let app = Router::new()
        .route(
            "/api/todo",
            get(api_get_todo)
                .post(api_post_todo)
                .put(api_put_todo)
                .delete(api_delete_todo),
        )
        .route("/api/todos", get(api_get_todos))
        .layer(cors_layer);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
