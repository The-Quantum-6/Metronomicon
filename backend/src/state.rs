use std::sync::Arc;

use postgres_es::{PostgresCqrs, PostgresViewRepository};

use crate::{aggregates::course::aggregate::Course, views::course::CourseView};

#[derive(Clone)]
pub struct AppState {
    pub cqrs: Arc<PostgresCqrs<Course>>,
    pub course_view_repo: Arc<PostgresViewRepository<CourseView, Course>>,
}
