use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::course::{command::CourseCommand, error::CourseError, event::CourseEvent};

#[derive(Serialize, Default, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub status: CourseStatus,
    pub name: String,
    pub code: String,
    pub field: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Default, Deserialize)]
pub enum CourseStatus {
    #[default]
    Active,
    Deleted,
}

#[async_trait]
impl Aggregate for Course {
    const TYPE: &'static str = "course";
    type Command = CourseCommand;
    type Event = CourseEvent;
    type Error = CourseError;
    type Services = ();

    fn handle(
        &mut self,
        command: Self::Command,
        service: &Self::Services,
        sink: &cqrs_es::event_sink::EventSink<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async { todo!() }
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
                self.status = CourseStatus::Active;
                self.name = name;
                self.code = code;
                self.field = field;
                self.description = description;
            }
            CourseEvent::CourseDeleted { .. } => self.status = CourseStatus::Deleted,
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
