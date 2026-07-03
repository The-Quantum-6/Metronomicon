use std::future::Future;

use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::{
    course::service::CourseServices,
    project_idea::{
        command::ProjectIdeaCommand, difficulty::Difficulty, error::ProjectIdeaError,
        event::ProjectIdeaEvent,
    },
    shared::Status,
};

pub struct ProjectIdeaAggregateServices {
    pub course: CourseServices,
}

#[derive(Serialize, Default, Deserialize)]
pub struct ProjectIdea {
    pub status: Status,
    pub idea_id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub body: String,
    pub difficulty: Option<Difficulty>,
}

impl Aggregate for ProjectIdea {
    const TYPE: &'static str = "project_idea";
    type Command = ProjectIdeaCommand;
    type Event = ProjectIdeaEvent;
    type Error = ProjectIdeaError;
    type Services = ProjectIdeaAggregateServices;

    fn handle(
        &mut self,
        command: Self::Command,
        service: &Self::Services,
        sink: &cqrs_es::event_sink::EventSink<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            match command {
                ProjectIdeaCommand::Create {
                    idea_id,
                    course_id,
                    title,
                    body,
                    difficulty,
                } => match self.status {
                    Status::Uninitialized => {
                        let course_exists = service
                            .course
                            .course_exists(&course_id.to_string())
                            .await
                            .unwrap();

                        if !course_exists {
                            Err("course not found".into())
                        } else {
                            let _: () = sink
                                .write(
                                    ProjectIdeaEvent::ProjectCreated {
                                        idea_id,
                                        course_id,
                                        title,
                                        body,
                                        difficulty,
                                    },
                                    self,
                                )
                                .await;
                            Ok(())
                        }
                    }
                    Status::Active | Status::Deleted => {
                        Err("This project_idea already exist".into())
                    }
                },
                ProjectIdeaCommand::Update {
                    idea_id,
                    title,
                    body,
                    difficulty,
                } => match self.status {
                    Status::Uninitialized => Err("Project_idea not found".into()),
                    Status::Deleted => Err("Cannot modify deleted project_idea".into()),
                    Status::Active => {
                        let _: () = sink
                            .write(
                                ProjectIdeaEvent::ProjectUpdated {
                                    idea_id,
                                    course_id
                                    title,
                                    body,
                                    difficulty,
                                },
                                self,
                            )
                            .await;
                        Ok(())
                    }
                },
                ProjectIdeaCommand::Delete { idea_id } => match self.status {
                    Status::Uninitialized => Err("Project idea not found".into()),
                    Status::Deleted => Err("Project idea already deleted".into()),
                    Status::Active => {
                        let _: () = sink
                            .write(
                                ProjectIdeaEvent::ProjectDeleted {
                                    idea_id,
                                    course_id: self.course_id,
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
            ProjectIdeaEvent::ProjectCreated {
                idea_id,
                course_id,
                title,
                body,
                difficulty,
            } => {
                self.status = Status::Active;
                self.idea_id = idea_id;
                self.course_id = course_id;
                self.title = title;
                self.body = body;
                self.difficulty = difficulty;
            }
            ProjectIdeaEvent::ProjectUpdated {
                title,
                body,
                difficulty,
                ..
            } => {
                if let Some(title) = title {
                    self.title = title;
                }
                if let Some(body) = body {
                    self.body = body;
                }
                if let Some(difficulty) = difficulty {
                    self.difficulty = Some(difficulty);
                }
            }
            ProjectIdeaEvent::ProjectDeleted { .. } => {
                self.status = Status::Deleted;
            }
        }
    }
}
