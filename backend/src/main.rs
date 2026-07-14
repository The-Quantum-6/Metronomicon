use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;

pub mod aggregates;
pub mod config;
pub mod error;
pub mod extractors;
pub mod models;
pub mod queries;
pub mod routes;
pub mod state;
pub mod storage;
pub mod views;
mod repositories;

#[tokio::main]
async fn main() {
    let config = config::get();

    // Builds the db pool (+ runs migrations), the Garage storage client and
    // the CQRS framework — everything handlers need, bundled in one AppState.
    let state = state::get(&config).await;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(routes::router())
        .with_state(state);

    // CORS off in dev
    let app = if config.cors_should_be_permissive {
        app.layer(CorsLayer::permissive())
    } else {
        app // no permissive layer outside dev
    };

    // Serve
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
