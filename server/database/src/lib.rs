use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{NewTodo, Todo};

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn create_todo(title: &str) -> Todo {
    use crate::schema::todos;
    let connection = &mut establish_connection();
    let new_todo = NewTodo { title };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(connection)
        .expect("Error saving new todo")
}

pub async fn get_todos() -> Vec<Todo> {
    use self::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    let results = todos
        .select(Todo::as_select())
        .load(connection)
        .expect("Error loading todos");
    results
}


pub async fn get_todo(id_to_find: i32) -> Todo {
    use self::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    let result = todos.find(id_to_find).get_result(connection).expect("Error getting ToDo");
    result
}

pub async fn update_todo(todo: Todo) -> Todo {
    use self::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    let results = diesel::update(todos.find(todo.id))
        .set((completed.eq(todo.completed), title.eq(todo.title)))
        .returning(Todo::as_returning())
        .get_result(connection)
        .expect("Error updating todo");
    results
}

pub fn delete_todo(id_to_delete: i32) {
    use self::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    diesel::delete(todos)
        .filter(id.eq(id_to_delete))
        .execute(connection)
        .expect("Error deleting todo");
}