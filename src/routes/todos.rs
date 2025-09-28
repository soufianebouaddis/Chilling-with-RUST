use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::{Todo, CreateTodo};

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .with_state(pool)
}

#[utoipa::path(
    get,
    path = "/api/todos",
    responses(
        (status = 200, description = "List all todos", body = [Todo])
    )
)]
async pub fn get_todos(State(pool): State<PgPool>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(todos)
}

#[utoipa::path(
    post,
    path = "/api/todos",
    request_body = CreateTodo,
    responses(
        (status = 201, description = "Todo created successfully", body = Todo)
    )
)]
async pub fn create_todo(State(pool): State<PgPool>, Json(payload): Json<CreateTodo>) -> Json<Todo> {
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (id, title, completed) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(payload.title)
    .bind(false)
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(todo)
}
