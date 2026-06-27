use std::sync::Arc;

use cqrs_es::{Query, persist::GenericQuery};
use postgres_es::{PostgresCqrs, PostgresViewRepository};
use sqlx::postgres::PgPoolOptions;

use crate::{
    aggregates::course::aggregate::Course, config::AppConfig, queries::test_logging_query,
    views::course::CourseView,
};

#[derive(Clone)]
pub struct AppState {
    pub cqrs: Arc<PostgresCqrs<Course>>,
    pub course_view_repo: Arc<PostgresViewRepository<CourseView, Course>>,
}

pub async fn get(config: &AppConfig) -> AppState {
    // Set up database connection
    let db = PgPoolOptions::new()
        .connect(&config.database_url)
        .await
        .expect("Should be able to connect to database");

    // Migrate database
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Migrations should succeed");

    // Queries setup
    type CourseViewRepo = PostgresViewRepository<CourseView, Course>;
    let course_view_repo: Arc<CourseViewRepo> =
        Arc::new(PostgresViewRepository::new("course_query", db.clone()));
    let mut course_query =
        GenericQuery::<CourseViewRepo, CourseView, Course>::new(course_view_repo.clone());
    course_query.use_error_handler(Box::new(|e| println!("{e}")));
    let logging_query = test_logging_query::SimpleLoggingQuery {};

    let queries: Vec<Box<dyn Query<Course>>> =
        vec![Box::new(course_query), Box::new(logging_query)];

    // CQRS framework
    let cqrs = Arc::new(postgres_es::postgres_cqrs(db, queries, ()));

    AppState {
        cqrs,
        course_view_repo,
    }
}
