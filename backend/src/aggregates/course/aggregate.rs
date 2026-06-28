use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::{
    course::{command::CourseCommand, error::CourseError, event::CourseEvent},
    shared::Status,
};

#[derive(Serialize, Default, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub status: Status,
    pub name: String,
    pub code: String,
    pub field: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl Aggregate for Course {
    const TYPE: &'static str = "course";
    type Command = CourseCommand;
    type Event = CourseEvent;
    type Error = CourseError;
    type Services = ();

    fn handle(
        &mut self,
        command: Self::Command,
        _service: &Self::Services,
        sink: &cqrs_es::event_sink::EventSink<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            match command {
                CourseCommand::Create {
                    id,
                    name,
                    code,
                    field,
                    description,
                } => match self.status {
                    Status::Uninitialized => {
                        let _: () = sink
                            .write(
                                CourseEvent::CourseCreated {
                                    id,
                                    name,
                                    code,
                                    field,
                                    description,
                                },
                                self,
                            )
                            .await;
                        Ok(())
                    }
                    _ => Err("course already exists".into()),
                },
                CourseCommand::Delete { .. } => match self.status {
                    Status::Uninitialized => Err("course not found".into()),
                    Status::Active => {
                        let _: () = sink
                            .write(CourseEvent::CourseDeleted { id: self.id }, self)
                            .await;
                        Ok(())
                    }
                    Status::Deleted => Err("course is already deleted".into()),
                },
                CourseCommand::UpdateMetadata {
                    name,
                    code,
                    field,
                    description,
                    ..
                } => match self.status {
                    Status::Uninitialized => Err("course not found".into()),
                    Status::Active => {
                        let _: () = sink
                            .write(
                                CourseEvent::CourseMetadataUpdated {
                                    id: self.id,
                                    name,
                                    code,
                                    field,
                                    description,
                                },
                                self,
                            )
                            .await;
                        Ok(())
                    }
                    Status::Deleted => Err("cannot modify deleted course".into()),
                },
                CourseCommand::AddTag { tag, .. } => match self.status {
                    Status::Uninitialized => Err("course not found".into()),
                    Status::Active => {
                        if self.tags.contains(&tag) {
                            Err("tag already exists".into())
                        } else {
                            let _: () = sink
                                .write(CourseEvent::TagAdded { id: self.id, tag }, self)
                                .await;
                            Ok(())
                        }
                    }
                    Status::Deleted => Err("cannot modify deleted course".into()),
                },
                CourseCommand::RemoveTag { tag, .. } => match self.status {
                    Status::Uninitialized => Err("course not found".into()),
                    Status::Active => {
                        if self.tags.contains(&tag) {
                            sink.write(CourseEvent::TagRemoved { id: self.id, tag }, self)
                                .await;
                            Ok(())
                        } else {
                            Err("tag not found".into())
                        }
                    }
                    Status::Deleted => Err("cannot modify deleted course".into()),
                },
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            CourseEvent::CourseCreated {
                id,
                name,
                code,
                field,
                description,
            } => {
                self.id = id;
                self.status = Status::Active;
                self.name = name;
                self.code = code;
                self.field = field;
                self.description = description;
            }
            CourseEvent::CourseDeleted { .. } => self.status = Status::Deleted,
            CourseEvent::CourseMetadataUpdated {
                name,
                code,
                field,
                description,
                ..
            } => {
                if let Some(name) = name {
                    self.name = name;
                }
                if let Some(code) = code {
                    self.code = code;
                }
                if let Some(field) = field {
                    self.field = field;
                }
                if let Some(description) = description {
                    self.description = description;
                }
            }
            CourseEvent::TagAdded { tag, .. } => {
                self.tags.push(tag);
            }
            CourseEvent::TagRemoved { tag, .. } => {
                self.tags.retain(|t| t != &tag);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cqrs_es::test::TestFramework;
    use uuid::Uuid;

    type CourseTestFramework = TestFramework<Course>;

    fn course_id() -> Uuid {
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
    }

    fn created_event() -> CourseEvent {
        CourseEvent::CourseCreated {
            id: course_id(),
            name: "Algorithms".into(),
            code: "CS301".into(),
            field: "Computer Science".into(),
            description: "Graph theory and beyond".into(),
        }
    }

    fn framework() -> CourseTestFramework {
        CourseTestFramework::with(())
    }

    // ── Create ────────────────────────────────────────────────────────────────

    #[test]
    fn test_create_course() {
        framework()
            .given_no_previous_events()
            .when(CourseCommand::Create {
                id: course_id(),
                name: "Algorithms".into(),
                code: "CS301".into(),
                field: "Computer Science".into(),
                description: "Graph theory and beyond".into(),
            })
            .then_expect_events(vec![created_event()]);
    }

    #[test]
    fn test_create_already_existing_course_returns_error() {
        framework()
            .given(vec![created_event()])
            .when(CourseCommand::Create {
                id: course_id(),
                name: "Algorithms".into(),
                code: "CS301".into(),
                field: "Computer Science".into(),
                description: "Graph theory and beyond".into(),
            })
            .then_expect_error_message("course already exists");
    }

    // ── Delete ────────────────────────────────────────────────────────────────

    #[test]
    fn test_delete_course() {
        framework()
            .given(vec![created_event()])
            .when(CourseCommand::Delete { id: course_id() })
            .then_expect_events(vec![CourseEvent::CourseDeleted { id: course_id() }]);
    }

    #[test]
    fn test_cannot_delete_uninitialized_course() {
        framework()
            .given_no_previous_events()
            .when(CourseCommand::Delete { id: course_id() })
            .then_expect_error_message("course not found");
    }

    #[test]
    fn test_delete_already_deleted_course_returns_error() {
        framework()
            .given(vec![
                created_event(),
                CourseEvent::CourseDeleted { id: course_id() },
            ])
            .when(CourseCommand::Delete { id: course_id() })
            .then_expect_error_message("course is already deleted");
    }

    // ── UpdateMetadata ────────────────────────────────────────────────────────

    #[test]
    fn test_update_metadata_partial() {
        framework()
            .given(vec![created_event()])
            .when(CourseCommand::UpdateMetadata {
                id: course_id(),
                name: Some("Algorithms II".into()),
                code: None,
                field: None,
                description: None,
            })
            .then_expect_events(vec![CourseEvent::CourseMetadataUpdated {
                id: course_id(),
                name: Some("Algorithms II".into()),
                code: None,
                field: None,
                description: None,
            }]);
    }

    #[test]
    fn test_cannot_update_uninitialized_course() {
        framework()
            .given_no_previous_events()
            .when(CourseCommand::UpdateMetadata {
                id: course_id(),
                name: Some("Algorithms II".into()),
                code: None,
                field: None,
                description: None,
            })
            .then_expect_error_message("course not found");
    }

    #[test]
    fn test_update_metadata_on_deleted_course_returns_error() {
        framework()
            .given(vec![
                created_event(),
                CourseEvent::CourseDeleted { id: course_id() },
            ])
            .when(CourseCommand::UpdateMetadata {
                id: course_id(),
                name: Some("Ghost".into()),
                code: None,
                field: None,
                description: None,
            })
            .then_expect_error_message("cannot modify deleted course");
    }

    // ── AddTag / RemoveTag ────────────────────────────────────────────────────

    #[test]
    fn test_add_tag() {
        framework()
            .given(vec![created_event()])
            .when(CourseCommand::AddTag {
                id: course_id(),
                tag: "graphs".into(),
            })
            .then_expect_events(vec![CourseEvent::TagAdded {
                id: course_id(),
                tag: "graphs".into(),
            }]);
    }

    #[test]
    fn test_cannot_add_tag_on_uninitialized_course() {
        framework()
            .given_no_previous_events()
            .when(CourseCommand::AddTag {
                id: course_id(),
                tag: "graphs".into(),
            })
            .then_expect_error_message("course not found");
    }

    #[test]
    fn test_add_duplicate_tag_returns_error() {
        framework()
            .given(vec![
                created_event(),
                CourseEvent::TagAdded {
                    id: course_id(),
                    tag: "graphs".into(),
                },
            ])
            .when(CourseCommand::AddTag {
                id: course_id(),
                tag: "graphs".into(),
            })
            .then_expect_error_message("tag already exists");
    }

    #[test]
    fn test_remove_tag() {
        framework()
            .given(vec![
                created_event(),
                CourseEvent::TagAdded {
                    id: course_id(),
                    tag: "graphs".into(),
                },
            ])
            .when(CourseCommand::RemoveTag {
                id: course_id(),
                tag: "graphs".into(),
            })
            .then_expect_events(vec![CourseEvent::TagRemoved {
                id: course_id(),
                tag: "graphs".into(),
            }]);
    }

    #[test]
    fn test_cannot_remove_tag_on_uninitialized_course() {
        framework()
            .given_no_previous_events()
            .when(CourseCommand::RemoveTag {
                id: course_id(),
                tag: "graphs".into(),
            })
            .then_expect_error_message("course not found");
    }

    #[test]
    fn test_remove_nonexistent_tag_returns_error() {
        framework()
            .given(vec![created_event()])
            .when(CourseCommand::RemoveTag {
                id: course_id(),
                tag: "nonexistent".into(),
            })
            .then_expect_error_message("tag not found");
    }
}
