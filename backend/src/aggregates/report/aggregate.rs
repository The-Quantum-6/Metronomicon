use std::future::Future;

use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::{
    course::service::CourseExistanceService,
    report::{
        command::ReportCommand,
        error::ReportError,
        event::ReportEvent,
        status::ReportStatus, // TODO: Add EmailService for validating email addresses
        target::ReportTarget,
    },
};

pub struct ReportAggregateServices {
    pub course: CourseExistanceService,
}

#[derive(Serialize, Default, Deserialize)]
pub struct Report {
    pub status: ReportStatus,
    pub issue_id: Uuid,
    pub target: Option<ReportTarget>,
    pub description: String,
    pub contact_email: Option<String>,
}

impl Aggregate for Report {
    const TYPE: &'static str = "report";

    type Command = ReportCommand;
    type Event = ReportEvent;
    type Error = ReportError;
    type Services = ReportAggregateServices;

    fn handle(
        &mut self,
        command: Self::Command,
        service: &Self::Services,
        sink: &cqrs_es::event_sink::EventSink<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            match command {
                ReportCommand::Create {
                    issue_id,
                    course_id,
                    title: _,
                    description,
                    contact_email,
                } => match self.status {
                    ReportStatus::Uninitialized => {
                        let target = match course_id {
                            Some(id) => {
                                let exists = service
                                    .course
                                    .course_exists(&id.to_string())
                                    .await
                                    .map_err(|_| "could not check course")?;

                                if !exists {
                                    return Err("course does not exist".into());
                                }

                                ReportTarget::Course(id)
                            }

                            None => ReportTarget::Site,
                        };

                        sink.write(
                            ReportEvent::ReportCreated {
                                issue_id,
                                target,
                                description,
                                contact_email,
                            },
                            self,
                        )
                        .await;

                        Ok(())
                    }

                    _ => Err("report already exists".into()),
                },

                ReportCommand::ResolveReport { issue_id } => match self.status {
                    ReportStatus::Open => {
                        sink.write(ReportEvent::ReportResolved { issue_id }, self)
                            .await;

                        Ok(())
                    }

                    ReportStatus::Resolved => Err("report already resolved".into()),

                    ReportStatus::Uninitialized => Err("report not found".into()),
                },

                ReportCommand::ReopenReport { issue_id } => match self.status {
                    ReportStatus::Resolved => {
                        sink.write(ReportEvent::ReportReopened { issue_id }, self)
                            .await;

                        Ok(())
                    }

                    ReportStatus::Open => Err("report already open".into()),

                    ReportStatus::Uninitialized => Err("report not found".into()),
                }, /*ReportCommand::Delete { issue_id } => match self.status {
                   ReportStatus::Uninitialized => Err("report not found".into()),
                   ReportStatus::Deleted => Err("report already deleted".into()),

                   ReportStatus::Open | ReportStatus::Resolved => {
                       sink
                           .write(ReportEvent::ReportDeleted { issue_id }, self)
                           .await;

                       Ok(())*/
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            ReportEvent::ReportCreated {
                issue_id,
                target,
                description,
                contact_email,
            } => {
                self.status = ReportStatus::Open;
                self.issue_id = issue_id;
                self.target = Some(target);
                self.description = description;
                self.contact_email = contact_email;
            }

            ReportEvent::ReportResolved { .. } => {
                self.status = ReportStatus::Resolved;
            }

            ReportEvent::ReportReopened { .. } => {
                self.status = ReportStatus::Open;
            } /*ReportEvent::ReportDeleted { .. } => {
                  self.status = ReportStatus::Deleted;
              }*/
        }
    }
}
