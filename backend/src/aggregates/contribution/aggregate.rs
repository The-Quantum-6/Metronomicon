use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::contribution::{
    command::{Contribution as ContributionKind, ContributionCommand, ModerationVerdict},
    error::ContributionError,
    event::ContributionEvent,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum ContributionStatus {
    #[default]
    Uninitialized,
    Proposed,
    Approved,
    Denied,
}

#[derive(Serialize, Default, Deserialize)]
pub struct ContributionAggregate {
    pub status: ContributionStatus,
    pub course_id: Uuid,
    pub contribution: Option<ContributionKind>,
}

impl Aggregate for ContributionAggregate {
    const TYPE: &'static str = "contribution";
    type Command = ContributionCommand;
    type Event = ContributionEvent;
    type Error = ContributionError;
    type Services = ();

    fn handle(
        &mut self,
        command: Self::Command,
        _service: &Self::Services,
        sink: &cqrs_es::event_sink::EventSink<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            match command {
                ContributionCommand::Propose {
                    contribution_id,
                    course_id,
                    contribution,
                } => match self.status {
                    ContributionStatus::Uninitialized => {
                        let _: () = sink
                            .write(
                                ContributionEvent::ContributionProposed {
                                    contribution_id,
                                    course_id,
                                    kind: contribution,
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
    use super::*;
    use crate::aggregates::contribution::command::TextContributionKind;
    use cqrs_es::test::TestFramework;
    use uuid::Uuid;

    type ContributionTestFramework = TestFramework<ContributionAggregate>;

    fn contribution_id() -> Uuid {
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
    }

    fn course_id() -> Uuid {
        Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()
    }

    fn framework() -> ContributionTestFramework {
        ContributionTestFramework::with(())
    }

    fn proposed_event() -> ContributionEvent {
        ContributionEvent::ContributionProposed {
            contribution_id: contribution_id(),
            course_id: course_id(),
            kind: ContributionKind::Text(TextContributionKind::AddLink {
                label: "Docs".into(),
                url: "https://example.com/docs".into(),
            }),
        }
    }

    fn proposed_contribution() -> ContributionKind {
        ContributionKind::Text(TextContributionKind::AddLink {
            label: "Docs".into(),
            url: "https://example.com/docs".into(),
        })
    }

    #[test]
    fn test_propose_contribution() {
        framework()
            .given_no_previous_events()
            .when(ContributionCommand::Propose {
                contribution_id: contribution_id(),
                course_id: course_id(),
                contribution: proposed_contribution(),
            })
            .then_expect_events(vec![proposed_event()]);
    }

    #[test]
    fn test_cannot_propose_existing_contribution() {
        framework()
            .given(vec![proposed_event()])
            .when(ContributionCommand::Propose {
                contribution_id: contribution_id(),
                course_id: course_id(),
                contribution: proposed_contribution(),
            })
            .then_expect_error_message("contribution already exists");
    }

    #[test]
    fn test_approve_contribution() {
        framework()
            .given(vec![proposed_event()])
            .when(ContributionCommand::Moderate {
                contribution_id: contribution_id(),
                verdict: ModerationVerdict::Approve,
            })
            .then_expect_events(vec![ContributionEvent::ContributionApproved {
                contribution_id: contribution_id(),
            }]);
    }

    #[test]
    fn test_deny_contribution() {
        framework()
            .given(vec![proposed_event()])
            .when(ContributionCommand::Moderate {
                contribution_id: contribution_id(),
                verdict: ModerationVerdict::Deny,
            })
            .then_expect_events(vec![ContributionEvent::ContributionDenied {
                contribution_id: contribution_id(),
            }]);
    }
}
