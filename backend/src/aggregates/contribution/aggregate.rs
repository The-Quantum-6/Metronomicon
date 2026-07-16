use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::{
    contribution::{
        command::{ContributionCommand, ContributionKind, ModerationVerdict, TextContributionKind},
        error::ContributionError,
        event::ContributionEvent,
    },
    course::service::CourseExistanceService,
    faq::services::FaqExistanceService,
    link::services::LinkServices, project_idea::services::ProjectIdeaExistanceService,
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
    pub faq: FaqExistanceService,
	pub project_idea: ProjectIdeaExistanceService,
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
                    contribution_id: _,
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

                        // Validate
                        match &contribution {
                            ContributionKind::Text(TextContributionKind::AddLink {
                                url,
                                label: _,
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
                                label: _,
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
                            ContributionKind::Text(TextContributionKind::AddFaqEntry {
                                question: _,
                                answer: _,
                            }) => (),
                            ContributionKind::Text(TextContributionKind::EditFaqEntry {
                                faq_id,
                                question: _,
                                answer: _,
                            }) => {
                                let exists = service
                                    .faq
                                    .faq_exists(&course_id.to_string(), faq_id)
                                    .await
                                    .map_err(|e| format!("database error: {}", e))?;
                                if !exists {
                                    return Err("link does not exist in course".into());
                                }
                            }
                            ContributionKind::Text(TextContributionKind::RemoveFaqEntry {
                                faq_id,
                            }) => {
                                let exists = service
                                    .faq
                                    .faq_exists(&course_id.to_string(), faq_id)
                                    .await
                                    .map_err(|e| format!("database error: {}", e))?;
                                if !exists {
                                    return Err("link does not exist in course".into());
                                }
                            }
							ContributionKind::Text(TextContributionKind::AddProjectIdea { title: _, body: _, difficulty: _ }) => (),
							ContributionKind::Text(TextContributionKind::EditProjectIdea { idea_id, title: _, body: _, difficulty: _ }) => {
								let exists = service
                                    .project_idea
                                    .project_idea_exists(&course_id.to_string(), idea_id)
                                    .await
                                    .map_err(|e| format!("database error: {}", e))?;
                                if !exists {
                                    return Err("project idea does not exist in course".into());
                                }
							}
							ContributionKind::Text(TextContributionKind::RemoveProjectIdea { idea_id }) => {
								let exists = service
                                    .project_idea
                                    .project_idea_exists(&course_id.to_string(), idea_id)
                                    .await
                                    .map_err(|e| format!("database error: {}", e))?;
                                if !exists {
                                    return Err("project idea does not exist in course".into());
                                }
							}
                            _ => todo!("Add support for project_idea and resource"),
                        }
                        let _: () = sink
                            .write(
                                ContributionEvent::ContributionProposed {
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
                    contribution_id: _,
                    verdict,
                } => match self.status {
                    ContributionStatus::Uninitialized => Err("contribution not found".into()),
                    ContributionStatus::Proposed => match verdict {
                        ModerationVerdict::Approve => {
                            let _: () = sink
                                .write(ContributionEvent::ContributionApproved, self)
                                .await;
                            Ok(())
                        }
                        ModerationVerdict::Deny => {
                            let _: () = sink
                                .write(ContributionEvent::ContributionDenied, self)
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
                course_id,
                kind,
                comment,
            } => {
                self.status = ContributionStatus::Proposed;
                self.course_id = course_id;
                self.contribution = Some(kind);
                self.comment = comment;
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
