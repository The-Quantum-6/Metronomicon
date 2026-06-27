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

#[tokio::main]
async fn main() {
    // Dotenv ok
    dotenvy::dotenv().ok();

    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Should be able to connect to database");

    // Migrate database
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Migrations should succeed");

    // Queries setup
    let course_view_repo: Arc<PostgresViewRepository<CourseView, Course>> =
        Arc::new(PostgresViewRepository::new("course_query", db.clone()));
    let mut course_query =
        GenericQuery::<PostgresViewRepository<CourseView, Course>, CourseView, Course>::new(
            course_view_repo.clone(),
        );
    course_query.use_error_handler(Box::new(|e| println!("{e}")));
    let logging_query = test_logging_query::SimpleLoggingQuery {};
    let queries: Vec<Box<dyn Query<Course>>> =
        vec![Box::new(course_query), Box::new(logging_query)];

    // CQRS framework
    let cqrs = Arc::new(postgres_es::postgres_cqrs(db.clone(), queries, ()));

    // AppState
    let state = AppState {
        cqrs,
        course_view_repo,
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(routes::router())
        .with_state(state);

    // CORS off in dev
    let environment = std::env::var("ENVIRONMENT").unwrap_or_default();
    let app = if environment == "dev" {
        app.layer(CorsLayer::permissive())
    } else {
        app // no permissive layer outside dev
    };

    // Serve
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
