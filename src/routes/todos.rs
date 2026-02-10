use axum::{
    extract::{State, Path},
    routing::get,
    Json, Router, http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::{Todo, CreateTodo, UpdateTodo};

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/:id", get(get_todo).put(update_todo).delete(delete_todo))
        .with_state(pool)
}

#[utoipa::path(
    get,
    path = "/api/todos",
    responses(
        (status = 200, description = "List all todos", body = [Todo])
    )
)]
pub async fn get_todos(State(pool): State<PgPool>) -> Json<Vec<Todo>> {
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
pub async fn create_todo(State(pool): State<PgPool>, Json(payload): Json<CreateTodo>) -> Json<Todo> {
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

#[utoipa::path(
    get,
    path = "/api/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Todo ID")
    ),
    responses(
        (status = 200, description = "Todo found", body = Todo),
        (status = 404, description = "Todo not found")
    )
)]
pub async fn get_todo(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Result<Json<Todo>, StatusCode> {
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .ok()
        .flatten();

    match todo {
        Some(t) => Ok(Json(t)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[utoipa::path(
    put,
    path = "/api/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Todo ID")
    ),
    request_body = UpdateTodo,
    responses(
        (status = 200, description = "Todo updated successfully", body = Todo),
        (status = 404, description = "Todo not found")
    )
)]
pub async fn update_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    // Fetch the existing todo first
    let existing = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .ok()
        .flatten();

    let existing = match existing {
        Some(t) => t,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // Use provided values or fallback to existing
    let title = payload.title.unwrap_or(existing.title);
    let completed = payload.completed.unwrap_or(existing.completed);

    let updated_todo = sqlx::query_as::<_, Todo>(
        "UPDATE todos SET title = $1, completed = $2 WHERE id = $3 RETURNING *",
    )
    .bind(title)
    .bind(completed)
    .bind(id)
    .fetch_one(&pool)
    .await
    .ok();

    match updated_todo {
        Some(t) => Ok(Json(t)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[utoipa::path(
    delete,
    path = "/api/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Todo ID")
    ),
    responses(
        (status = 204, description = "Todo deleted successfully"),
        (status = 404, description = "Todo not found")
    )
)]
pub async fn delete_todo(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .ok();

    match result {
        Some(r) if r.rows_affected() > 0 => Ok(StatusCode::NO_CONTENT),
        _ => Err(StatusCode::NOT_FOUND),
    }
}
