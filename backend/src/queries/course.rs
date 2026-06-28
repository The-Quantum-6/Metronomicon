use std::sync::Arc;

use cqrs_es::persist::GenericQuery;
use postgres_es::PostgresViewRepository;
use sqlx::{Pool, Postgres};

use crate::{
    aggregates::course::aggregate::Course,
    views::course::{CourseDetailView, CourseDetailViewRepo},
};

pub type CourseQuery =
    GenericQuery<PostgresViewRepository<CourseDetailView, Course>, CourseDetailView, Course>;

pub fn get(pool: Pool<Postgres>) -> (Arc<CourseDetailViewRepo>, CourseQuery) {
    let course_view_repo: Arc<CourseDetailViewRepo> =
        Arc::new(PostgresViewRepository::new("course_detail_view", pool));
    let mut course_query: CourseQuery = CourseQuery::new(course_view_repo.clone());
    course_query.use_error_handler(Box::new(|e| println!("{e}")));

    (course_view_repo, course_query)
}
