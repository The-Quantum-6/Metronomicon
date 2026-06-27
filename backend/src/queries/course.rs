use cqrs_es::persist::GenericQuery;
use postgres_es::PostgresViewRepository;

use crate::{aggregates::course::aggregate::Course, views::course::CourseView};

pub type CourseQuery = GenericQuery<PostgresViewRepository<CourseView, Course>, CourseView, Course>;
