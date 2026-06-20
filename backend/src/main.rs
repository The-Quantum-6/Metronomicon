mod routes;

use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

pub mod models;
mod repositories;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // ok() delibaretely discards any error with dotenv. This avoids 'unused result'-warning(which would trigger without it) and allows the code to work with no .env file present (i.e.: in docker, where env vars are directly injected)

    /* DB setup */
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Should be able to connect to database");
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Migrations should succeed");

    /* Main app */
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(routes::router())
        .with_state(db);

    /* Disable CORS in dev(no proxy) means backend rejects frontend otherwise*/
    let environment = std::env::var("ENVIRONMENT").unwrap_or_default();

    let app = if environment == "dev" {
        app.layer(CorsLayer::permissive())
    } else {
        app // no permissive layer outside dev
    };

    /* Serve */
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
