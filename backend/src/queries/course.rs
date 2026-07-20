use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query, persist::GenericQuery};
use postgres_es::PostgresViewRepository;
use sqlx::{Pool, Postgres};

use crate::{
    aggregates::course::{aggregate::Course, event::CourseEvent},
    views::course::active_detailed::CourseDetailView,
};

pub type CourseQuery =
    GenericQuery<PostgresViewRepository<CourseDetailView, Course>, CourseDetailView, Course>;

pub struct CourseListQuery {
    pool: Pool<Postgres>,
}

impl CourseListQuery {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Query<Course> for CourseListQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Course>]) {
        for event in events {
            let result = match &event.payload {
                CourseEvent::CourseCreated {
                    name, code, field, ..
                } => {
                    sqlx::query!(
                        "INSERT INTO course_list_view 
                         (aggregate_id, name, code, field, status)
                         VALUES ($1, $2, $3, $4, 'Active')
                         ON CONFLICT (aggregate_id) DO UPDATE
                         SET name = $2, code = $3, field = $4",
                        aggregate_id,
                        name,
                        code,
                        field
                    )
                    .execute(&self.pool)
                    .await
                }
                CourseEvent::CourseDeleted => {
                    sqlx::query!(
                        "UPDATE course_list_view SET status = 'Deleted'
                         WHERE aggregate_id = $1",
                        aggregate_id
                    )
                    .execute(&self.pool)
                    .await
                }
                CourseEvent::CourseMetadataUpdated {
                    name, code, field, ..
                } => {
                    sqlx::query!(
                        "UPDATE course_list_view
                         SET name = COALESCE($2, name),
                             code = COALESCE($3, code),
                             field = COALESCE($4, field)
                         WHERE aggregate_id = $1",
                        aggregate_id,
                        name.as_deref(),
                        code.as_deref(),
                        field.as_deref(),
                    )
                    .execute(&self.pool)
                    .await
                }
                _ => continue,
            };

            if let Err(e) = result {
                println!("CourseListQuery error: {e}");
            }
        }
    }
}
