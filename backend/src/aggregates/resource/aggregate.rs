use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::{
    course::service::CourseServices,
    resource::{
        command::ResourceCommand, error::ResourceError, event::ResourceEvent,
        services::ResourceServices,
    },
    shared::{Officiality, Status},
};

pub struct ResourceAggregateServices {
    pub course: CourseServices,
    pub resource: ResourceServices,
}

#[derive(Serialize, Default, Deserialize)]
pub struct Resource {
    pub status: Status,
    pub officiality: Officiality,
    pub resource_id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub key: String,
}

impl Aggregate for Resource {
    const TYPE: &'static str = "resource";
    type Command = ResourceCommand;
    type Event = ResourceEvent;
    type Error = ResourceError;
    type Services = ResourceAggregateServices;

    fn handle(
        &mut self,
        command: Self::Command,
        service: &Self::Services,
        sink: &cqrs_es::event_sink::EventSink<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            match command {
                ResourceCommand::Create {
                    resource_id,
                    course_id,
                    title,
                    key,
                } => match self.status {
                    Status::Uninitialized => {
                        match service
                            .course
                            .course_exists(&course_id.to_string())
                            .await
                            .unwrap()
                        {
                            false => Err("course not found".into()),
                            true => match service.resource.check_exists(&key).await {
                                Err(_) => Err("resource file not found".into()),
                                Ok(()) => {
                                    let _: () = sink
                                        .write(
                                            ResourceEvent::ResourceCreated {
                                                resource_id,
                                                course_id,
                                                title,
                                                key,
                                            },
                                            self,
                                        )
                                        .await;
                                    Ok(())
                                }
                            },
                        }
                    }
                    _ => Err("resource already exists".into()),
                },
                ResourceCommand::Update {
                    resource_id,
                    course_id,
                    title,
                    key,
                } => match self.status {
                    Status::Uninitialized => Err("resource not found".into()),
                    Status::Deleted => Err("cannot modify deleted resource".into()),
                    Status::Active => {
                        let _: () = sink
                            .write(
                                ResourceEvent::ResourceUpdated {
                                    resource_id,
                                    course_id,
                                    title,
                                    key,
                                },
                                self,
                            )
                            .await;
                        Ok(())
                    }
                },
                ResourceCommand::Delete {
                    resource_id,
                    course_id,
                } => match self.status {
                    Status::Uninitialized => Err("resource not found".into()),
                    Status::Deleted => Err("resource already deleted".into()),
                    Status::Active => {
                        let _: () = sink
                            .write(
                                ResourceEvent::ResourceDeleted {
                                    resource_id,
                                    course_id,
                                },
                                self,
                            )
                            .await;
                        Ok(())
                    }
                },
                ResourceCommand::SetOfficial {
                    resource_id,
                    course_id,
                    officiality,
                } => match self.status {
                    Status::Uninitialized => Err("resource not found".into()),
                    Status::Deleted => Err("resource already deleted".into()),
                    Status::Active => {
                        let _: () = sink
                            .write(
                                ResourceEvent::ResourceOfficialStatusChanged {
                                    resource_id,
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
            ResourceEvent::ResourceCreated {
                resource_id,
                course_id,
                title,
                key,
            } => {
                self.status = Status::Active;
                self.resource_id = resource_id;
                self.course_id = course_id;
                self.title = title;
                self.key = key;
            }
            ResourceEvent::ResourceUpdated { title, key, .. } => {
                if let Some(title) = title {
                    self.title = title;
                }
                if let Some(key) = key {
                    self.key = key;
                }
            }
            ResourceEvent::ResourceDeleted { .. } => {
                self.status = Status::Deleted;
            }
            ResourceEvent::ResourceOfficialStatusChanged { officiality, .. } => {
                self.officiality = officiality;
            }
        }
    }
}
