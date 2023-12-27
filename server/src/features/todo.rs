use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use database::{create_todo, delete_todo, get_todo, get_todos, models::Todo, update_todo};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct TodoDto {
    id: i32,
    title: String,
    completed: bool,
}
impl From<Todo> for TodoDto {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            completed: todo.completed,
        }
    }
}
impl From<TodoDto> for Todo {
    fn from(todo: TodoDto) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            completed: todo.completed,
        }
    }
}

pub async fn api_get_todos() -> impl IntoResponse {
    let todos = get_todos().await;
    let dto: Vec<TodoDto> = todos.into_iter().map(|todo| todo.into()).collect();
    (StatusCode::OK, Json(dto))
}

pub async fn api_put_todo(Json(todo): Json<TodoDto>) -> impl IntoResponse {
    let updated_todo = update_todo(Todo::from(todo)).await;
    let dto = TodoDto::from(updated_todo);
    (StatusCode::OK, Json(dto))
}

#[derive(Deserialize)]
pub struct CreateTodoDto {
    title: String,
}
pub async fn api_post_todo(Json(todo): Json<CreateTodoDto>) -> impl IntoResponse {
    let body: String = todo.title.to_string();
    let created_todo = create_todo(&body).await;
    let dto = TodoDto::from(created_todo);
    (StatusCode::OK, Json(dto))
}

#[derive(Deserialize)]
pub struct GetTodoRequest {
    id: i32,
}
pub async fn api_get_todo(req: Query<GetTodoRequest>) -> impl IntoResponse {
    let todo = get_todo(req.id).await;
    let dto = TodoDto::from(todo);
    (StatusCode::OK, Json(dto))
}

#[derive(Deserialize)]
pub struct DeleteTodoRequesst {
    id: i32,
}
pub async fn api_delete_todo(Json(todo): Json<DeleteTodoRequesst>) -> impl IntoResponse {
    let id = todo.id;
    let deleted = delete_todo(id);
    (StatusCode::OK, Json(deleted))
}
