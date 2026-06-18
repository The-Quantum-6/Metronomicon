use axum::{
    Router, extract::State, routing::get
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

pub mod models;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new().connect(&database_url).await.expect("Should be able to connect to database");

    sqlx::migrate!().run(&db).await.expect("Migrations should succeed");

    let environment = std::env::var("ENVIRONMENT").unwrap_or_default();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(db);

    let app = if environment == "dev" {
        app.layer(CorsLayer::permissive())
    } else {
        app // no permissive layer outside dev
    };
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}