use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query};
use sqlx::{Pool, Postgres};

use crate::aggregates::contribution::{aggregate::Contribution, event::ContributionEvent};

pub struct ContributionQuery;

#[async_trait]
impl Query<Contribution> for ContributionQuery {
    async fn dispatch(
        &self,
        _contribution_id: &str,
        _events: &[EventEnvelope<Contribution>],
    ) {
        // lightweight logging query kept for compatibility; actual DB projection
        // is implemented in `ContributionListQuery` below.
    }
}

pub struct ContributionListQuery {
    pool: Pool<Postgres>,
}

impl ContributionListQuery {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Query<Contribution> for ContributionListQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Contribution>]) {
        for event in events {
            let result = match &event.payload {
                ContributionEvent::ContributionProposed {
                    course_id, kind, ..
                } => {
                    // serialize the contribution payload to JSONB
                    let contribution_json =
                        serde_json::to_value(kind).unwrap_or(serde_json::Value::Null);
                    sqlx::query(
                        "INSERT INTO contribution_list_view (aggregate_id, course_id, contribution, status)
                         VALUES ($1, $2, $3::jsonb, 'Proposed')
                         ON CONFLICT (aggregate_id) DO UPDATE
                         SET course_id = $2, contribution = $3::jsonb, status = 'Proposed'",
                    )
                    .bind(aggregate_id)
                    .bind(course_id.to_string())
                    .bind(contribution_json)
                    .execute(&self.pool)
                    .await
                }
                ContributionEvent::ContributionApproved { .. } => sqlx::query(
                    "UPDATE contribution_list_view SET status = 'Approved' WHERE aggregate_id = $1",
                )
                .bind(aggregate_id)
                .execute(&self.pool)
                .await,
                ContributionEvent::ContributionDenied { .. } => sqlx::query(
                    "UPDATE contribution_list_view SET status = 'Denied' WHERE aggregate_id = $1",
                )
                .bind(aggregate_id)
                .execute(&self.pool)
                .await,
            };

            if let Err(e) = result {
                println!("ContributionListQuery error: {e}");
            }
        }
    }
}
