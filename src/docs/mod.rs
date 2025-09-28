
use utoipa::OpenApi;
use crate::routes::todos::{get_todos, create_todo};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::todos::get_todos,
        crate::routes::todos::create_todo,
    ),
    components(
        schemas(crate::model::Todo)
    )
)]
pub struct ApiDoc;