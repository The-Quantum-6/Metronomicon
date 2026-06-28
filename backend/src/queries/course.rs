use std::sync::Arc;

use cqrs_es::persist::GenericQuery;
use postgres_es::PostgresViewRepository;
use sqlx::{Pool, Postgres};

use crate::{
    aggregates::course::aggregate::Course,
    views::course::{CourseView, CourseViewRepo},
};

pub type CourseQuery = GenericQuery<PostgresViewRepository<CourseView, Course>, CourseView, Course>;

pub fn get(pool: Pool<Postgres>) -> (Arc<CourseViewRepo>, CourseQuery) {
    let course_view_repo: Arc<CourseViewRepo> =
        Arc::new(PostgresViewRepository::new("course_query", pool));
    let mut course_query: CourseQuery = CourseQuery::new(course_view_repo.clone());
    course_query.use_error_handler(Box::new(|e| println!("{e}")));

    (course_view_repo, course_query)
}
