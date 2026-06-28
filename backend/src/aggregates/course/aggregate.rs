use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use crate::aggregates::{
    course::{command::CourseCommand, error::CourseError, event::CourseEvent},
    shared::Status,
};

/// The `Course` aggregate — the central building block for all course-related behaviour.
///
/// An aggregate is the gatekeeper for a piece of domain state. All changes go
/// through it; nothing mutates the state directly.
///
/// # How it works
///
/// 1. A client sends a [`CourseCommand`].
/// 2. [`handle`](Aggregate::handle) validates it against the current state and,
///    if valid, writes one or more [`CourseEvent`]s via the event sink.
/// 3. Each committed event is immediately passed to [`apply`](Aggregate::apply),
///    which updates the in-memory state.
///
/// # What counts as "invalid"
///
/// A command is invalid when the *current state* makes it impossible to execute —
/// e.g. creating a course that already exists. This is distinct from a command
/// that simply can't be parsed (handle that at the type level instead).
#[derive(Serialize, Default, Deserialize)]
pub struct Course {
    pub status: Status,
    pub name: String,
    pub code: String,
    pub field: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl Aggregate for Course {
    /// Identifies this aggregate type in the event store. Must be globally unique.
    const TYPE: &'static str = "course";
    type Command = CourseCommand; // Commands may be issued by clients
    type Event = CourseEvent; // Events may be generated if the command is valid
    type Error = CourseError; // Errors may be returned if they are not
    type Services = (); // The course aggregate has no external services for the moment

    /// Validates a command and, if accepted, writes the resulting event(s).
    ///
    /// This is where all business logic lives. Before adding validation here,
    /// consider the following priority order:
    ///
    /// 1. **Prefer type-level validation.** If a value is always invalid (e.g. a
    ///    negative count), use an unsigned type in the command so it fails at
    ///    deserialization — no logic needed here.
    /// 2. **Use services for external checks.** If you need to validate against
    ///    data outside this aggregate (e.g. "does this course code already exist
    ///    globally?"), inject a service. Think carefully before adding one.
    /// 3. **Everything else goes here.** Validation against the aggregate's own
    ///    state (e.g. "is this course already deleted?") belongs in this function.
    fn handle(
        &mut self,
        command: Self::Command,
        _service: &Self::Services,
        sink: &cqrs_es::event_sink::EventSink<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            match command {
                CourseCommand::Create {
                    name,
                    code,
                    field,
                    description,
                    ..
                } => match self.status {
                    Status::Uninitialized => {
                        let _: () = sink
                            .write(
                                CourseEvent::CourseCreated {
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
                        let _: () = sink.write(CourseEvent::CourseDeleted, self).await;
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
                            let _: () = sink.write(CourseEvent::TagAdded { tag }, self).await;
                            Ok(())
                        }
                    }
                    Status::Deleted => Err("cannot modify deleted course".into()),
                },
                CourseCommand::RemoveTag { tag, .. } => match self.status {
                    Status::Uninitialized => Err("course not found".into()),
                    Status::Active => {
                        if self.tags.contains(&tag) {
                            sink.write(CourseEvent::TagRemoved { tag }, self).await;
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

    /// Applies a committed event to update the aggregate's in-memory state.
    ///
    /// This function **cannot fail**. Events are immutable facts — by the time
    /// `apply` is called, the event is already persisted. Any validation that
    /// could produce an error must happen in [`handle`](Aggregate::handle) before
    /// the event is written.
    ///
    /// `apply` is called in two situations:
    /// - Immediately after `handle` writes an event (live path).
    /// - If the aggregate is rebuilt for some reason, replaying events from start.
    fn apply(&mut self, event: Self::Event) {
        match event {
            CourseEvent::CourseCreated {
                name,
                code,
                field,
                description,
            } => {
                self.status = Status::Active;
                self.name = name;
                self.code = code;
                self.field = field;
                self.description = description;
            }
            CourseEvent::CourseDeleted => self.status = Status::Deleted,
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
                // No duplicate check here — that's `handle`s job. Once an event
                // is committed it is treated as correct.
                self.tags.push(tag);
            }
            CourseEvent::TagRemoved { tag, .. } => {
                // Same principle: no existence check. `handle` already verified
                // the tag was present before writing this event.
                self.tags.retain(|t| t != &tag);
            }
        }
    }
}

// Event sourcing makes unit tests unusually clean: each test is a short story
// of the form "given these past events, when this command arrives, expect this
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
                course_id: course_id(),
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
                course_id: course_id(),
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
            .when(CourseCommand::Delete {
                course_id: course_id(),
            })
            .then_expect_events(vec![CourseEvent::CourseDeleted]);
    }

    #[test]
    fn test_cannot_delete_uninitialized_course() {
        framework()
            .given_no_previous_events()
            .when(CourseCommand::Delete {
                course_id: course_id(),
            })
            .then_expect_error_message("course not found");
    }

    #[test]
    fn test_delete_already_deleted_course_returns_error() {
        framework()
            .given(vec![created_event(), CourseEvent::CourseDeleted])
            .when(CourseCommand::Delete {
                course_id: course_id(),
            })
            .then_expect_error_message("course is already deleted");
    }

    // ── UpdateMetadata ────────────────────────────────────────────────────────

    #[test]
    fn test_update_metadata_partial() {
        framework()
            .given(vec![created_event()])
            .when(CourseCommand::UpdateMetadata {
                course_id: course_id(),
                name: Some("Algorithms II".into()),
                code: None,
                field: None,
                description: None,
            })
            .then_expect_events(vec![CourseEvent::CourseMetadataUpdated {
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
                course_id: course_id(),
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
            .given(vec![created_event(), CourseEvent::CourseDeleted])
            .when(CourseCommand::UpdateMetadata {
                course_id: course_id(),
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
                course_id: course_id(),
                tag: "graphs".into(),
            })
            .then_expect_events(vec![CourseEvent::TagAdded {
                tag: "graphs".into(),
            }]);
    }

    #[test]
    fn test_cannot_add_tag_on_uninitialized_course() {
        framework()
            .given_no_previous_events()
            .when(CourseCommand::AddTag {
                course_id: course_id(),
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
                    tag: "graphs".into(),
                },
            ])
            .when(CourseCommand::AddTag {
                course_id: course_id(),
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
                    tag: "graphs".into(),
                },
            ])
            .when(CourseCommand::RemoveTag {
                course_id: course_id(),
                tag: "graphs".into(),
            })
            .then_expect_events(vec![CourseEvent::TagRemoved {
                tag: "graphs".into(),
            }]);
    }

    #[test]
    fn test_cannot_remove_tag_on_uninitialized_course() {
        framework()
            .given_no_previous_events()
            .when(CourseCommand::RemoveTag {
                course_id: course_id(),
                tag: "graphs".into(),
            })
            .then_expect_error_message("course not found");
    }

    #[test]
    fn test_remove_nonexistent_tag_returns_error() {
        framework()
            .given(vec![created_event()])
            .when(CourseCommand::RemoveTag {
                course_id: course_id(),
                tag: "nonexistent".into(),
            })
            .then_expect_error_message("tag not found");
    }
}
