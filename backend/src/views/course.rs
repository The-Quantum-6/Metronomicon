use cqrs_es::View;
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};

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
        match &event.payload {
            LinkEvent::LinkCreated {
                link_id,
                course_id: _,
                label,
                url,
            } => {
                self.links.push(LinkDetailView {
                    link_id: link_id.clone(),
                    status: Status::Active,
                    label: label.clone(),
                    url: url.clone(),
                    official: false,
                });
            }
            LinkEvent::LinkUpdated {
                link_id,
                label,
                url,
                ..
            } => {
                let l = self
                    .links
                    .iter_mut()
                    .find(|l| l.link_id == *link_id)
                    .unwrap();
                if let Some(label) = label {
                    l.label = label.clone();
                }
                if let Some(url) = url {
                    l.url = url.clone();
                }
            }
            LinkEvent::LinkDeleted { link_id, .. } => {
                self.links.retain(|l| &l.link_id != link_id);
            }
            LinkEvent::LinkOfficialStatusChanged {
                link_id, official, ..
            } => {
                let l = self
                    .links
                    .iter_mut()
                    .find(|l| l.link_id == *link_id)
                    .unwrap();
                l.official = *official;
            }
        }
    }
}
