use axum::{
    Router, extract::State, routing::get
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new().connect(&database_url).await.expect("Should be able to connect to database");

    sqlx::migrate!().run(&db).await.expect("Migrations should succeed");

    let environment = std::env::var("ENVIRONMENT").unwrap_or_default();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/upload", get(upload))
        .route("/fetch", get(fetch))
        .with_state(db);

    let app = if environment == "dev" {
        app.layer(CorsLayer::permissive())
    } else {
        app // no permissive layer outside dev
    };
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn upload(State(db): State<sqlx::PgPool>) -> &'static str {
    sqlx::query("INSERT INTO test (content) VALUES ($1)")
        .bind("hello from upload endpoint")
        .execute(&db)
        .await
        .expect("Insert should succeed");

    "Uploaded!"
}

async fn fetch(State(db): State<sqlx::PgPool>) -> String {
    let rows: Vec<(Option<String>,)> = sqlx::query_as("SELECT content FROM test")
        .fetch_all(&db)
        .await
        .expect("Fetch should succeed");

    rows.iter()
        .map(|(content,)| content.clone().unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n")
}