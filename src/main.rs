use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::net::SocketAddr;


mod routes;
mod model;
mod docs;
mod db;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let pool = db::init_db().await?;
    let app = Router::new()
        .merge(routes::todos::router(pool.clone()))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", docs::ApiDoc::openapi()));

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    tracing::info!("🚀 Server running at http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
