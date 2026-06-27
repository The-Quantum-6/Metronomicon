use cqrs_es::View;
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::course::{
    aggregate::{Course, CourseStatus},
    event::CourseEvent,
};

pub type CourseViewRepo = PostgresViewRepository<CourseView, Course>;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct CourseView {
    pub id: Uuid,
    pub status: CourseStatus,
    pub name: String,
    pub code: String,
    pub field: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl View<Course> for CourseView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Course>) {
        match &event.payload {
            CourseEvent::CourseCreated {
                id,
                name,
                code,
                field,
                description,
            } => {
                self.id = *id;
                self.status = CourseStatus::Active;
                self.name = name.clone();
                self.code = code.clone();
                self.field = field.clone();
                self.description = description.clone();
            }
            CourseEvent::CourseDeleted { id: _ } => {
                self.status = CourseStatus::Deleted;
            }
            CourseEvent::CourseMetadataUpdated {
                id: _,
                name,
                code,
                field,
                description,
            } => {
                if let Some(name) = name {
                    self.name = name.clone();
                }
                if let Some(code) = code {
                    self.code = code.clone();
                }
                if let Some(field) = field {
                    self.field = field.clone();
                }
                if let Some(description) = description {
                    self.description = description.clone();
                }
            }
            CourseEvent::TagAdded { id: _, tag } => {
                self.tags.push(tag.clone());
            }
            CourseEvent::TagRemoved { id: _, tag } => {
                self.tags.retain(|t| t != tag);
            }
        }
    }
}
