use axum::Router;
use sqlx::PgPool;

pub mod todos;

pub fn router(pool: PgPool) -> Router {
    Router::new().nest("/api", todos::router(pool))
}
