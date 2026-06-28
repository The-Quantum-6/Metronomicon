use std::sync::Arc;

use cqrs_es::{
    View,
    persist::{PersistenceError, ViewRepository},
};
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    aggregates::{
        course::{aggregate::Course, event::CourseEvent},
        link::{aggregate::Link, event::LinkEvent},
        shared::Status,
    },
    views::link::LinkDetailView,
};

pub type CourseDetailViewRepo = PostgresViewRepository<CourseDetailView, Course>;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct CourseDetailView {
    pub status: Status,
    pub name: String,
    pub code: String,
    pub field: String,
    pub description: String,
    pub tags: Vec<String>,
    pub links: Vec<LinkDetailView>,
}

impl View<Course> for CourseDetailView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Course>) {
        match &event.payload {
            CourseEvent::CourseCreated {
                name,
                code,
                field,
                description,
            } => {
                self.status = Status::Active;
                self.name = name.clone();
                self.code = code.clone();
                self.field = field.clone();
                self.description = description.clone();
            }
            CourseEvent::CourseDeleted => {
                self.status = Status::Deleted;
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

impl View<Link> for CourseDetailView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Link>) {
        let link_id = Uuid::parse_str(&event.aggregate_id).unwrap();
        match &event.payload {
            LinkEvent::LinkCreated { label, url, .. } => {
                self.links.push(LinkDetailView {
                    link_id,
                    status: Status::Active,
                    label: label.clone(),
                    url: url.clone(),
                    official: false,
                });
            }
            LinkEvent::LinkUpdated { label, url, .. } => {
                let l = self
                    .links
                    .iter_mut()
                    .find(|l| l.link_id == link_id)
                    .unwrap();
                if let Some(label) = label {
                    l.label = label.clone();
                }
                if let Some(url) = url {
                    l.url = url.clone();
                }
            }
            LinkEvent::LinkDeleted { .. } => {
                self.links.retain(|l| l.link_id != link_id);
            }
            LinkEvent::LinkOfficialStatusChanged { official, .. } => {
                let l = self
                    .links
                    .iter_mut()
                    .find(|l| l.link_id == link_id)
                    .unwrap();
                l.official = *official;
            }
        }
    }
}

#[derive(Clone)]
pub struct ActiveCourseViewRepo(pub Arc<CourseDetailViewRepo>);

impl ActiveCourseViewRepo {
    pub async fn load_active(
        &self,
        id: &str,
    ) -> Result<Option<CourseDetailView>, PersistenceError> {
        match self.0.load(id).await? {
            Some(view) if view.status == Status::Active => Ok(Some(view)),
            Some(_) => Ok(None), // deleted = not found, from caller's perspective
            None => Ok(None),
        }
    }
}
