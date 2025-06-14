use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

mod agents;
mod config;
mod models;
mod routes;
use config::Settings;

fn create_router() -> Router {
    Router::new()
        .route("/", get(routes::root::get))
        .route("/health", get(routes::health::get))
        .route("/query", post(routes::query::post))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO),
                )
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                ),
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let settings = Settings::new();
    let addr = format!("{}:{}", settings.host, settings.port);
    let socket_addr: SocketAddr = addr.parse()?;

    let app = create_router();

    let listener = tokio::net::TcpListener::bind(socket_addr).await?;
    tracing::info!("running on http://{}", socket_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
