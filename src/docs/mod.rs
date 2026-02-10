
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::todos::get_todos,
        crate::routes::todos::create_todo,
        crate::routes::todos::get_todo,
        crate::routes::todos::update_todo,
        crate::routes::todos::delete_todo,
    ),
    components(
        schemas(crate::model::Todo, crate::model::CreateTodo, crate::model::UpdateTodo)
    )
)]
pub struct ApiDoc;