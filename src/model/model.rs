use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: Option<String>,
    pub completed: Option<bool>,
}
