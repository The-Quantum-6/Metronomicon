use cqrs_es::View;
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};

use crate::aggregates::report::{
    aggregate::Report, event::ReportEvent, status::ReportStatus, target::ReportTarget,
};

pub type ReportDetailViewRepo = PostgresViewRepository<ReportDetailView, Report>;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct ReportDetailView {
    pub status: ReportStatus,
    pub target: Option<ReportTarget>,
    pub description: String,
    pub contact_email: Option<String>,
}

impl View<Report> for ReportDetailView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Report>) {
        match &event.payload {
            ReportEvent::ReportCreated {
                target,
                description,
                contact_email,
                ..
            } => {
                self.status = ReportStatus::Open;
                self.target = Some(target.clone());
                self.description = description.clone();
                self.contact_email = contact_email.clone();
            }
            ReportEvent::ReportResolved { .. } => {
                self.status = ReportStatus::Resolved;
            }
            ReportEvent::ReportReopened { .. } => {
                self.status = ReportStatus::Open;
            }
        }
    }
}
