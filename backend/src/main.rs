mod auth;
mod handlers;
mod routes;
mod state;

use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tower_http::cors::CorsLayer;

use tower_sessions::{Expiry, SessionManagerLayer, cookie::{SameSite, time::Duration}};
use tower_sessions_sqlx_store::PostgresStore;
pub mod error;
pub mod middleware;
pub mod models;
mod repositories;
mod storage;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Should be able to connect to database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Migrations should succeed");

    let session_store = PostgresStore::new(db.clone());
    session_store.migrate().await.unwrap();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::days(30)));

    let environment = std::env::var("ENVIRONMENT").unwrap_or_default();

    let state = AppState::new(db).await;
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(routes::router(state.clone()))
        .layer(session_layer)
        .with_state(state);
    

    let app = if environment == "dev" {
        app.layer(CorsLayer::permissive())
    } else {
        app // no permissive layer outside dev
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
