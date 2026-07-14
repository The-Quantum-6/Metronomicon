use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use strum::Display;
use uuid::Uuid;

use crate::aggregates::report::target::ReportTarget;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Display)]
pub enum ReportEvent {
    ReportCreated {
        issue_id: Uuid,
        target: ReportTarget,
        description: String,
        contact_email: Option<String>,
    },
    ReportResolved {
        issue_id: Uuid,
    },
    ReportReopened {
        issue_id: Uuid,
    },
}

impl DomainEvent for ReportEvent {
    fn event_type(&self) -> String {
        self.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}