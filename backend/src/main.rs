mod auth;
mod handlers;
mod routes;
mod state;

use axum::{Router, http::HeaderValue, routing::get};
use reqwest::{Method, header::CONTENT_TYPE};
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tower_http::cors::CorsLayer;

use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use tower_sessions_sqlx_store::PostgresStore;
pub mod aggregates;
pub mod config;
pub mod error;
pub mod extractors;
pub mod middleware;
pub mod models;
pub mod queries;
mod repositories;
pub mod storage;
pub mod views;

#[tokio::main]
async fn main() {
    let config = config::get();

    let state = state::get(&config).await;
    let db = state.pool.clone();

    let session_store = PostgresStore::new(db.clone());
    session_store.migrate().await.unwrap();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::days(30)));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(routes::router())
        .merge(routes::protected_router(state.clone()))
        .merge(routes::authed_router(state.clone()))
        .layer(session_layer)
        .with_state(state);

    let app = if config.cors_should_be_permissive {
        let cors = CorsLayer::new()
            .allow_origin(config.frontend_url.parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
            .allow_headers([CONTENT_TYPE])
            .allow_credentials(true); // PÅKREVD når fetch har credentials: "include"

        app.layer(cors)
    } else {
        app
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
