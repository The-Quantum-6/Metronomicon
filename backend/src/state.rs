use std::sync::Arc;

use cqrs_es::Query;
use postgres_es::{PostgresCqrs, PostgresViewRepository};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::{
    aggregates::{
        course::{aggregate::Course, service::CourseServices},
        link::{
            aggregate::{Link, LinkAggregateServices},
            services::LinkServices,
        },
    },
    config::AppConfig,
    queries::{
        course::{CourseListQuery, CourseQuery},
        link::CourseLinkQuery,
        test_logging_query,
    },
    views::course::active_detailed::{ActiveCourseViewRepo, CourseDetailViewRepo},
};

#[derive(Clone)]
pub struct AppState {
    pub cqrs: Arc<Cqrs>,
    pub course_view_repo: ActiveCourseViewRepo,
    pub pool: Pool<Postgres>,
}

#[derive(Clone)]
pub struct Cqrs {
    pub course: Arc<PostgresCqrs<Course>>,
    pub link: Arc<PostgresCqrs<Link>>,
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
    let logging_query = test_logging_query::SimpleLoggingQuery {};

    let course_view_repo: Arc<CourseDetailViewRepo> = Arc::new(PostgresViewRepository::new(
        "course_detail_view",
        db.clone(),
    ));
    let mut course_detail_query: CourseQuery = CourseQuery::new(course_view_repo.clone());
    course_detail_query.use_error_handler(Box::new(|e| println!("{e}")));
    let course_list_query = CourseListQuery::new(db.clone());

    let course_queries: Vec<Box<dyn Query<Course>>> = vec![
        Box::new(course_detail_query),
        Box::new(course_list_query),
        Box::new(logging_query.clone()),
    ];
    let course_cqrs = Arc::new(postgres_es::postgres_cqrs(db.clone(), course_queries, ()));

    let link_queries: Vec<Box<dyn Query<Link>>> = vec![
        Box::new(logging_query),
        Box::new(CourseLinkQuery::new(course_view_repo.clone())),
    ];
    let link_aggregate_services = LinkAggregateServices {
        course: CourseServices(db.clone()),
        link: LinkServices(reqwest::Client::new()),
    };
    let link_cqrs = Arc::new(postgres_es::postgres_cqrs(
        db.clone(),
        link_queries,
        link_aggregate_services,
    ));

    AppState {
        cqrs: Arc::new(Cqrs {
            course: course_cqrs,
            link: link_cqrs,
        }),
        course_view_repo: ActiveCourseViewRepo(course_view_repo),
        pool: db,
    }
}
