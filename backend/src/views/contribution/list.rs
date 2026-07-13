use cqrs_es::View;
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::contribution::{
    aggregate::{ContributionAggregate, ContributionStatus},
    event::ContributionEvent,
};

pub type ContributionListViewRepo =
    PostgresViewRepository<ContributionListView, ContributionAggregate>;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct ContributionListView {
    pub aggregate_id: Uuid,
    pub course_id: Uuid,
    pub contribution: serde_json::Value,
    pub status: ContributionStatus,
}

impl View<ContributionAggregate> for ContributionListView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<ContributionAggregate>) {
        match &event.payload {
            ContributionEvent::ContributionProposed {
                contribution_id,
                course_id,
                kind,
            } => {
                self.aggregate_id = *contribution_id;
                self.course_id = *course_id;
                self.contribution = serde_json::to_value(kind).unwrap_or(serde_json::Value::Null);
                self.status = ContributionStatus::Proposed;
            }
            ContributionEvent::ContributionApproved { .. } => {
                self.status = ContributionStatus::Approved;
            }
            ContributionEvent::ContributionDenied { .. } => {
                self.status = ContributionStatus::Denied;
            }
        }
    }
}
