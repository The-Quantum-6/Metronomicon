use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::{
    link::{command::LinkCommand, error::LinkError, event::LinkEvent, services::LinkServices},
    shared::Status,
};

#[derive(Serialize, Default, Deserialize)]
pub struct Link {
    pub link_id: Uuid,
    pub status: Status,
    pub course_id: Uuid,
    pub label: String,
    pub url: String,
    pub official: bool,
}

impl Aggregate for Link {
    const TYPE: &'static str = "link";
    type Command = LinkCommand;
    type Event = LinkEvent;
    type Error = LinkError;
    type Services = LinkServices;

    fn handle(
        &mut self,
        command: Self::Command,
        _service: &Self::Services,
        sink: &cqrs_es::event_sink::EventSink<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            match command {
                LinkCommand::Create {
                    link_id,
                    course_id,
                    label,
                    url,
                } => match self.status {
                    Status::Uninitialized => {
                        let _: () = sink
                            .write(
                                LinkEvent::LinkCreated {
                                    link_id,
                                    course_id,
                                    label,
                                    url,
                                },
                                self,
                            )
                            .await;
                        Ok(())
                    }
                    _ => Err("link already exists".into()),
                },
                LinkCommand::Update { label, url, .. } => match self.status {
                    Status::Uninitialized => Err("link not found".into()),
                    Status::Deleted => Err("Cannot modify deleted link".into()),
                    Status::Active => {
                        let _: () = sink
                            .write(
                                LinkEvent::LinkUpdated {
                                    link_id: self.link_id,
                                    course_id: self.course_id,
                                    label,
                                    url,
                                },
                                self,
                            )
                            .await;
                        Ok(())
                    }
                },
                LinkCommand::Delete { .. } => match self.status {
                    Status::Uninitialized => Err("link not found".into()),
                    Status::Deleted => Err("link already deleted".into()),
                    Status::Active => {
                        let _: () = sink
                            .write(
                                LinkEvent::LinkDeleted {
                                    link_id: self.link_id,
                                    course_id: self.course_id,
                                },
                                self,
                            )
                            .await;
                        Ok(())
                    }
                },
                LinkCommand::SetOfficial { official, .. } => match self.status {
                    Status::Uninitialized => Err("link not found".into()),
                    Status::Deleted => Err("link already deleted".into()),
                    Status::Active => {
                        let _: () = sink
                            .write(
                                LinkEvent::LinkOfficialStatusChanged {
                                    link_id: self.link_id,
                                    course_id: self.course_id,
                                    official,
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
            LinkEvent::LinkCreated {
                link_id,
                course_id,
                label,
                url,
            } => {
                self.link_id = link_id;
                self.status = Status::Active;
                self.course_id = course_id;
                self.label = label;
                self.url = url;
                self.official = false;
            }
            LinkEvent::LinkUpdated { label, url, .. } => {
                if let Some(label) = label {
                    self.label = label;
                }
                if let Some(url) = url {
                    self.url = url;
                }
            }
            LinkEvent::LinkDeleted { .. } => {
                self.status = Status::Deleted;
            }
            LinkEvent::LinkOfficialStatusChanged { official, .. } => {
                self.official = official;
            }
        }
    }
}
