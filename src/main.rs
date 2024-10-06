mod db_connect;
mod error;
mod image_service;
mod models;
mod controllers;

use std::net::SocketAddr;

use axum::{response::{Html, IntoResponse}, routing::{get, post}, Extension, Router};
use image_service::get_image;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::trace::TraceLayer;


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let pool = db_connect::db_connect().await;
    
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
        .route("/home", get(get_home))
        .route("/images/:uuid", get(get_image))
        .route("/blogposts", get(controllers::blogpost::get_all))
        .route("/blogposts", post(controllers::blogpost::process_create_blogpost_request))
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("Listening on {}", addr);


    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, app).await?;

    Ok(())

}

async fn get_home() -> impl IntoResponse {
    Html(include_str!("views/home_view.html"))
}