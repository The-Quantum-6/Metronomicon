use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query, persist::GenericQuery};
use postgres_es::PostgresViewRepository;
use sqlx::{Pool, Postgres};

use crate::{
    aggregates::report::{aggregate::Report, event::ReportEvent},
    views::admin::report_detail::ReportDetailView,
};

pub type ReportQuery =
    GenericQuery<PostgresViewRepository<ReportDetailView, Report>, ReportDetailView, Report>;

pub struct ReportListQuery {
    pool: Pool<Postgres>,
}

impl ReportListQuery {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Query<Report> for ReportListQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Report>]) {
        for event in events {
            let result =
                match &event.payload {
                    ReportEvent::ReportCreated {
                        target,
                        description,
                        contact_email,
                        ..
                    } => {
                        sqlx::query(
                            "INSERT INTO report_list_view
                         (aggregate_id, target, description, contact_email, status)
                         VALUES ($1, $2, $3, $4, 'Open')
                         ON CONFLICT (aggregate_id) DO UPDATE
                         SET target = $2, description = $3, contact_email = $4",
                        )
                        .bind(aggregate_id)
                        .bind(format!("{target:?}"))
                        .bind(description)
                        .bind(contact_email.as_deref())
                        .execute(&self.pool)
                        .await
                    }
                    ReportEvent::ReportResolved { .. } => sqlx::query(
                        "UPDATE report_list_view SET status = 'Resolved' WHERE aggregate_id = $1",
                    )
                    .bind(aggregate_id)
                    .execute(&self.pool)
                    .await,
                    ReportEvent::ReportReopened { .. } => {
                        sqlx::query(
                            "UPDATE report_list_view SET status = 'Open' WHERE aggregate_id = $1",
                        )
                        .bind(aggregate_id)
                        .execute(&self.pool)
                        .await
                    }
                };

            if let Err(e) = result {
                println!("ReportListQuery error: {e}");
            }
        }
    }
}
