use crate::aggregates::course::aggregate::CourseStatus;
use cqrs_es::View;
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::course::{aggregate::Course, event::CourseEvent};

pub type CourseListViewRepo = PostgresViewRepository<CourseListView, Course>;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct CourseListView {
    pub id: Uuid,
    pub status: CourseStatus,
    pub name: String,
    pub code: String,
    pub field: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl View<Course> for CourseListView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Course>) {
        match &event.payload {
            CourseEvent::CourseCreated {
                name,
                code,
                field,
                description,
            } => {
                self.status = CourseStatus::Active;
                self.name = name.clone();
                self.code = code.clone();
                self.field = field.clone();
                self.description = description.clone();
            }
            CourseEvent::CourseUnactivated => {
                self.status = CourseStatus::Unactive;
            }
            CourseEvent::CourseActivated => {
                self.status = CourseStatus::Active;
            }
            CourseEvent::CourseMetadataUpdated {
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
            CourseEvent::TagAdded { tag } => {
                self.tags.push(tag.clone());
            }
            CourseEvent::TagRemoved { tag } => {
                self.tags.retain(|t| t != tag);
            }
        }
    }
}
