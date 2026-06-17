mod auth;
mod routes;
mod state;

use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tower_http::cors::CorsLayer;

use tower_sessions::{SessionManagerLayer, cookie::SameSite};
use tower_sessions_sqlx_store::PostgresStore;

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
        .with_same_site(SameSite::Lax);

    let environment = std::env::var("ENVIRONMENT").unwrap_or_default();

    let state = AppState::new(db).await;
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(routes::router())
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
