use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::{
    contribution::{
        command::{
            Contribution as ContributionKind, ContributionCommand, ModerationVerdict,
            TextContributionKind,
        },
        error::ContributionError,
        event::ContributionEvent,
    },
    course::service::CourseExistanceService,
    link::services::LinkServices,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum ContributionStatus {
    #[default]
    Uninitialized,
    Proposed,
    Approved,
    Denied,
}

pub struct ContributionAggregateServices {
    pub link: LinkServices,
    pub course: CourseExistanceService,
}

#[derive(Serialize, Default, Deserialize)]
pub struct Contribution {
    pub status: ContributionStatus,
    pub course_id: Uuid,
    pub contribution: Option<ContributionKind>,
    pub comment: String,
}

impl Aggregate for Contribution {
    const TYPE: &'static str = "contribution";
    type Command = ContributionCommand;
    type Event = ContributionEvent;
    type Error = ContributionError;
    type Services = ContributionAggregateServices;

    fn handle(
        &mut self,
        command: Self::Command,
        service: &Self::Services,
        sink: &cqrs_es::event_sink::EventSink<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            match command {
                ContributionCommand::Propose {
                    contribution_id,
                    course_id,
                    contribution,
                    comment,
                } => match self.status {
                    ContributionStatus::Uninitialized => {
                        // Ensure the course exists before accepting contributions
                        let course_exists = service
                            .course
                            .course_exists(&course_id.to_string())
                            .await
                            .map_err(|e| format!("database error: {}", e))?;
                        if !course_exists {
                            return Err("course not found".into());
                        }

                        // Validate link-related contributions
                        match &contribution {
                            ContributionKind::Text(TextContributionKind::AddLink {
                                url, ..
                            }) => {
                                service
                                    .link
                                    .0
                                    .check_valid(url)
                                    .await
                                    .map_err(|e| format!("link validation error: {}", e))?;
                            }
                            ContributionKind::Text(TextContributionKind::EditLink {
                                link_id,
                                url,
                                ..
                            }) => {
                                let exists = service
                                    .link
                                    .1
                                    .link_exists(&course_id.to_string(), link_id)
                                    .await
                                    .map_err(|e| format!("database error: {}", e))?;
                                if !exists {
                                    return Err("link does not exist in course".into());
                                }
                                if let Some(new_url) = url {
                                    service
                                        .link
                                        .0
                                        .check_valid(new_url)
                                        .await
                                        .map_err(|e| format!("link validation error: {}", e))?;
                                }
                            }
                            ContributionKind::Text(TextContributionKind::RemoveLink {
                                link_id,
                            }) => {
                                let exists = service
                                    .link
                                    .1
                                    .link_exists(&course_id.to_string(), link_id)
                                    .await
                                    .map_err(|e| format!("database error: {}", e))?;
                                if !exists {
                                    return Err("link does not exist in course".into());
                                }
                            }
                            _ => {}
                        }
                        let _: () = sink
                            .write(
                                ContributionEvent::ContributionProposed {
                                    contribution_id,
                                    course_id,
                                    kind: contribution,
                                    comment,
                                },
                                self,
                            )
                            .await;
                        Ok(())
                    }
                    _ => Err("contribution already exists".into()),
                },
                ContributionCommand::Moderate {
                    contribution_id,
                    verdict,
                } => match self.status {
                    ContributionStatus::Uninitialized => Err("contribution not found".into()),
                    ContributionStatus::Proposed => match verdict {
                        ModerationVerdict::Approve => {
                            let _: () = sink
                                .write(
                                    ContributionEvent::ContributionApproved { contribution_id },
                                    self,
                                )
                                .await;
                            Ok(())
                        }
                        ModerationVerdict::Deny => {
                            let _: () = sink
                                .write(
                                    ContributionEvent::ContributionDenied { contribution_id },
                                    self,
                                )
                                .await;
                            Ok(())
                        }
                    },
                    ContributionStatus::Approved => Err("contribution is already approved".into()),
                    ContributionStatus::Denied => Err("contribution is already denied".into()),
                },
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            ContributionEvent::ContributionProposed {
                course_id, kind, ..
            } => {
                self.status = ContributionStatus::Proposed;
                self.course_id = course_id;
                self.contribution = Some(kind);
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

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    #[allow(dead_code)]
    fn contribution_id() -> Uuid {
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
    }

    #[allow(dead_code)]
    fn course_id() -> Uuid {
        Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()
    }

    // Note: Tests for command handling require async services and are tested via integration tests.
    // Unit tests for apply (state transitions) would go here if needed.
}
