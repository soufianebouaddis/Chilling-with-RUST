use utoipa::OpenApi;

use crate::model::{Todo, CreateTodo, UpdateTodo};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::todos::get_todos,
        crate::routes::todos::create_todo,
        crate::routes::todos::get_todo,
        crate::routes::todos::update_todo,
        crate::routes::todos::delete_todo,
    ),
    components(schemas(Todo, CreateTodo, UpdateTodo)),
    tags(
        (name = "todo", description = "Todo management endpoints")
    )
)]
pub struct ApiDoc;
