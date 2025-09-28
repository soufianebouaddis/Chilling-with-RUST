use utoipa::OpenApi;

use crate::models::Todo;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::todos::get_todos,
        crate::routes::todos::create_todo,
    ),
    components(schemas(Todo)),
    tags(
        (name = "todo", description = "Todo management endpoints")
    )
)]
pub struct ApiDoc;
