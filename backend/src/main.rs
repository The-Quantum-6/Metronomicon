use std::sync::Arc;

use axum::{Router, routing::get};
use cqrs_es::{Query, persist::GenericQuery};
use postgres_es::PostgresViewRepository;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

use crate::{
    aggregates::course::aggregate::Course, queries::test_logging_query, state::AppState,
    views::course::CourseView,
};

pub mod aggregates;
pub mod error;
pub mod extractors;
pub mod models;
pub mod queries;
pub mod repositories;
pub mod routes;
pub mod state;
pub mod views;
pub mod config;

#[tokio::main]
async fn main() {
    let config = config::get();
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
