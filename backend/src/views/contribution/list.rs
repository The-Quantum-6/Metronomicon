use cqrs_es::View;
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::contribution::{
    aggregate::{Contribution, ContributionStatus},
    event::ContributionEvent,
};

pub type ContributionListViewRepo = PostgresViewRepository<ContributionListView, Contribution>;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct ContributionListView {
    pub aggregate_id: Uuid,
    pub course_id: Uuid,
    pub contribution: serde_json::Value,
    pub status: ContributionStatus,
    pub comment: String,
}

impl View<Contribution> for ContributionListView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Contribution>) {
        match &event.payload {
            ContributionEvent::ContributionProposed {
                course_id,
                kind,
                comment,
            } => {
                self.aggregate_id = Uuid::parse_str(&event.aggregate_id).unwrap();
                self.course_id = *course_id;
                self.contribution = serde_json::to_value(kind).unwrap_or(serde_json::Value::Null);
                self.status = ContributionStatus::Proposed;
                self.comment = comment.clone();
            }
            ContributionEvent::ContributionApproved => {
                self.status = ContributionStatus::Approved;
            }
            ContributionEvent::ContributionDenied => {
                self.status = ContributionStatus::Denied;
            }
        }
    }
}
