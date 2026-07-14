use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::{
    faq::{command::FaqCommand, error::FaqError, event::FaqEvent, service::FaqAggregateServices},
    shared::{Officiality, Status},
};

#[derive(Serialize, Default, Deserialize)]
pub struct Faq {
    pub status: Status,
    pub officiality: Officiality,
    pub faq_id: Uuid,
    pub course_id: Uuid,
    pub question: String,
    pub answer: String,
}

impl Aggregate for Faq {
    const TYPE: &'static str = "faq";
    type Command = FaqCommand;
    type Event = FaqEvent;
    type Error = FaqError;
    type Services = FaqAggregateServices;

    fn handle(
        &mut self,
        command: Self::Command,
        service: &Self::Services,
        sink: &cqrs_es::event_sink::EventSink<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            match command {
                FaqCommand::Create {
                    faq_id,
                    course_id,
                    question,
                    answer,
                    ..
                } => match self.status {
                    Status::Uninitialized => {
                        match service
                            .course
                            .course_exists(&course_id.to_string())
                            .await
                            .unwrap()
                        {
                            false => Err("course not found".into()),
                            true => {
                                let _: () = sink
                                    .write(
                                        FaqEvent::FaqCreated {
                                            faq_id,
                                            course_id,
                                            question,
                                            answer,
                                        },
                                        self,
                                    )
                                    .await;
                                Ok(())
                            }
                        }
                    }
                    _ => Err("Faq already exists".into()),
                },
                FaqCommand::Delete { faq_id, course_id } => match self.status {
                    Status::Uninitialized => Err("Faq not found".into()),
                    Status::Active => {
                        let _: () = sink
                            .write(FaqEvent::FaqDeleted { faq_id, course_id }, self)
                            .await;
                        Ok(())
                    }
                    Status::Deleted => Err("Faq is already deleted".into()),
                },
                FaqCommand::Update {
                    faq_id,
                    course_id,
                    question,
                    answer,
                } => match self.status {
                    Status::Uninitialized => Err("Faq not found".into()),
                    Status::Active => {
                        let _: () = sink
                            .write(
                                FaqEvent::FaqUpdated {
                                    faq_id,
                                    course_id,
                                    question,
                                    answer,
                                },
                                self,
                            )
                            .await;
                        Ok(())
                    }
                    Status::Deleted => Err("cannot modify deleted Faq".into()),
                },

                FaqCommand::SetOfficial {
                    course_id,
                    officiality,
                    ..
                } => match self.status {
                    Status::Uninitialized => Err("Faq not found".into()),
                    Status::Deleted => Err("Faq is already deleted".into()),
                    Status::Active => {
                        let _: () = sink
                            .write(
                                FaqEvent::FaqOfficialStatusChanged {
                                    course_id,
                                    officiality,
                                },
                                self,
                            )
                            .await;
                        Ok(())
                    }
                },
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            FaqEvent::FaqCreated {
                faq_id,
                course_id,
                question,
                answer,
            } => {
                self.status = Status::Active;
                self.faq_id = faq_id;
                self.course_id = course_id;
                self.question = question;
                self.answer = answer;
            }
            FaqEvent::FaqDeleted { .. } => self.status = Status::Deleted,
            FaqEvent::FaqUpdated {
                question, answer, ..
            } => {
                if let Some(question) = question {
                    self.question = question;
                }
                if let Some(answer) = answer {
                    self.answer = answer;
                }
            }
            FaqEvent::FaqOfficialStatusChanged { officiality, .. } => {
                self.officiality = officiality;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregates::course::service::CourseExistanceService;
    use cqrs_es::test::TestFramework;
    use uuid::Uuid;

    type FaqTestFramework = TestFramework<Faq>;

    fn faq_id() -> Uuid {
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
    }

    fn course_id() -> Uuid {
        Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()
    }

    fn created_event() -> FaqEvent {
        FaqEvent::FaqCreated {
            faq_id: faq_id(),
            course_id: course_id(),
            question: "What is Big-O notation?".into(),
            answer: "A way to describe an algorithm's growth rate.".into(),
        }
    }

    fn framework() -> FaqTestFramework {
        FaqTestFramework::with(FaqAggregateServices {
            course: CourseExistanceService(
                sqlx::PgPool::connect_lazy("postgres://invalid/invalid")
                    .expect("connect_lazy should not require a live connection"),
            ),
        })
    }

    // ── Create ────────────────────────────────────────────────────────────────

    #[test]
    #[ignore = "hits CourseExistanceService::course_exists; needs a live Postgres connection"]
    fn test_create_faq() {
        framework()
            .given_no_previous_events()
            .when(FaqCommand::Create {
                faq_id: faq_id(),
                course_id: course_id(),
                question: "What is Big-O notation?".into(),
                answer: "A way to describe an algorithm's growth rate.".into(),
            })
            .then_expect_events(vec![created_event()]);
    }

    #[test]
    #[ignore = "hits CourseExistanceService::course_exists; needs a live Postgres connection"]
    fn test_cannot_create_faq_for_nonexistent_course() {
        framework()
            .given_no_previous_events()
            .when(FaqCommand::Create {
                faq_id: faq_id(),
                course_id: course_id(),
                question: "What is Big-O notation?".into(),
                answer: "A way to describe an algorithm's growth rate.".into(),
            })
            .then_expect_error_message("course not found");
    }

    #[test]
    fn test_create_already_existing_faq_returns_error() {
        // Already-existing check happens before the course-existence check,
        // so this never touches the database — safe to run offline.
        framework()
            .given(vec![created_event()])
            .when(FaqCommand::Create {
                faq_id: faq_id(),
                course_id: course_id(),
                question: "What is Big-O notation?".into(),
                answer: "A way to describe an algorithm's growth rate.".into(),
            })
            .then_expect_error_message("Faq already exists");
    }

    // ── Delete ────────────────────────────────────────────────────────────────

    #[test]
    fn test_delete_faq() {
        framework()
            .given(vec![created_event()])
            .when(FaqCommand::Delete {
                faq_id: faq_id(),
                course_id: course_id(),
            })
            .then_expect_events(vec![FaqEvent::FaqDeleted {
                faq_id: faq_id(),
                course_id: course_id(),
            }]);
    }

    #[test]
    fn test_cannot_delete_uninitialized_faq() {
        framework()
            .given_no_previous_events()
            .when(FaqCommand::Delete {
                faq_id: faq_id(),
                course_id: course_id(),
            })
            .then_expect_error_message("Faq not found");
    }

    #[test]
    fn test_delete_already_deleted_faq_returns_error() {
        framework()
            .given(vec![
                created_event(),
                FaqEvent::FaqDeleted {
                    faq_id: faq_id(),
                    course_id: course_id(),
                },
            ])
            .when(FaqCommand::Delete {
                faq_id: faq_id(),
                course_id: course_id(),
            })
            .then_expect_error_message("Faq is already deleted");
    }

    // ── Update ───────────────────────────────────────────────────────────────

    #[test]
    fn test_update_partial() {
        framework()
            .given(vec![created_event()])
            .when(FaqCommand::Update {
                faq_id: faq_id(),
                course_id: course_id(),
                question: Some("What is Big-O, really?".into()),
                answer: None,
            })
            .then_expect_events(vec![FaqEvent::FaqUpdated {
                faq_id: faq_id(),
                course_id: course_id(),
                question: Some("What is Big-O, really?".into()),
                answer: None,
            }]);
    }

    #[test]
    fn test_cannot_update_uninitialized_faq() {
        framework()
            .given_no_previous_events()
            .when(FaqCommand::Update {
                faq_id: faq_id(),
                course_id: course_id(),
                question: Some("What is Big-O, really?".into()),
                answer: None,
            })
            .then_expect_error_message("Faq not found");
    }

    #[test]
    fn test_update_on_deleted_faq_returns_error() {
        framework()
            .given(vec![
                created_event(),
                FaqEvent::FaqDeleted {
                    faq_id: faq_id(),
                    course_id: course_id(),
                },
            ])
            .when(FaqCommand::Update {
                faq_id: faq_id(),
                course_id: course_id(),
                question: Some("Ghost question".into()),
                answer: None,
            })
            .then_expect_error_message("cannot modify deleted Faq");
    }

    // ── SetOfficial ──────────────────────────────────────────────────────────

    #[test]
    fn test_set_official() {
        framework()
            .given(vec![created_event()])
            .when(FaqCommand::SetOfficial {
                faq_id: faq_id(),
                course_id: course_id(),
                officiality: Officiality::Official,
            })
            .then_expect_events(vec![FaqEvent::FaqOfficialStatusChanged {
                course_id: course_id(),
                officiality: Officiality::Official,
            }]);
    }

    #[test]
    fn test_cannot_set_official_on_uninitialized_faq() {
        framework()
            .given_no_previous_events()
            .when(FaqCommand::SetOfficial {
                faq_id: faq_id(),
                course_id: course_id(),
                officiality: Officiality::Official,
            })
            .then_expect_error_message("Faq not found");
    }

    #[test]
    fn test_cannot_set_official_on_deleted_faq() {
        framework()
            .given(vec![
                created_event(),
                FaqEvent::FaqDeleted {
                    faq_id: faq_id(),
                    course_id: course_id(),
                },
            ])
            .when(FaqCommand::SetOfficial {
                faq_id: faq_id(),
                course_id: course_id(),
                officiality: Officiality::Official,
            })
            .then_expect_error_message("Faq is already deleted");
    }
}
