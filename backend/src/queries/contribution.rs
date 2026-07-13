use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query};
use uuid::Uuid;

use crate::{
    aggregates::contribution::{
        aggregate::{ContributionAggregate, ContributionStatus},
        event::ContributionEvent,
    },
    views::contribution::ContributionDetailView,
};

pub struct ContributionQuery;

#[async_trait]
impl Query<ContributionAggregate> for ContributionQuery {
    async fn dispatch(
        &self,
        contribution_id: &str,
        events: &[EventEnvelope<ContributionAggregate>],
    ) {
        let contribution_id = Uuid::parse_str(contribution_id).unwrap();
        let mut view = ContributionDetailView {
            status: ContributionStatus::Uninitialized,
            contribution_id,
            course_id: Uuid::nil(),
            contribution: None,
        };

        for event in events {
            match &event.payload {
                ContributionEvent::ContributionProposed {
                    contribution_id: id,
                    course_id,
                    kind,
                } => {
                    view.contribution_id = *id;
                    view.course_id = *course_id;
                    view.status = ContributionStatus::Proposed;
                    view.contribution = Some(kind.clone());
                    println!(
                        "ContributionQuery: contribution {} proposed for course {}",
                        id, course_id
                    );
                }
                ContributionEvent::ContributionApproved { contribution_id: id } => {
                    view.status = ContributionStatus::Approved;
                    println!("ContributionQuery: contribution {} approved", id);
                }
                ContributionEvent::ContributionDenied { contribution_id: id } => {
                    view.status = ContributionStatus::Denied;
                    println!("ContributionQuery: contribution {} denied", id);
                }
            }
        }
    }
}
