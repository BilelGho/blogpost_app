mod db_connect;
mod error;
mod image_service;
mod models;

use std::net::SocketAddr;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use image_service::get_image;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::trace::TraceLayer;


#[tokio::main]
async fn main() -> anyhow::Result<()> {


    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    let app = Router::new()
        .route("/", get(root))
        .route("/images/:uuid", get(get_image))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("Listening on {}", addr);


    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, app).await?;

    Ok(())

}

async fn root() -> impl IntoResponse {
    (StatusCode::OK,"Hello, World!")
}