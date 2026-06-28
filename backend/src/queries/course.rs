use cqrs_es::persist::GenericQuery;
use postgres_es::PostgresViewRepository;

use crate::{aggregates::course::aggregate::Course, views::course::active_detailed::CourseDetailView};

pub type CourseQuery =
    GenericQuery<PostgresViewRepository<CourseDetailView, Course>, CourseDetailView, Course>;
