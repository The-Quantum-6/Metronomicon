use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use strum::Display;
use uuid::Uuid;

use crate::aggregates::contribution::command::ContributionKind;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Display)]
pub enum ContributionEvent {
    ContributionProposed {
        course_id: Uuid,
        kind: ContributionKind,
        comment: String,
    },
    ContributionApproved,
    ContributionDenied,
}

impl DomainEvent for ContributionEvent {
    fn event_type(&self) -> String {
        self.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
