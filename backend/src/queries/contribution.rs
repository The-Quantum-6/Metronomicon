use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query};
use postgres_es::PostgresCqrs;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use crate::aggregates::{
    contribution::{
        aggregate::Contribution,
        command::{ContributionKind, TextContributionKind},
        event::ContributionEvent,
    },
    faq::{aggregate::Faq, command::FaqCommand},
    link::{aggregate::Link, command::LinkCommand},
    project_idea::{aggregate::ProjectIdea, command::ProjectIdeaCommand},
};

pub struct ContributionQuery;

#[async_trait]
impl Query<Contribution> for ContributionQuery {
    async fn dispatch(&self, _contribution_id: &str, _events: &[EventEnvelope<Contribution>]) {
        // lightweight logging query kept for compatibility; actual DB projection
        // is implemented in `ContributionListQuery` below.
    }
}

pub struct ContributionListQuery {
    pool: Pool<Postgres>,
}

impl ContributionListQuery {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Query<Contribution> for ContributionListQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Contribution>]) {
        for event in events {
            let result = match &event.payload {
                ContributionEvent::ContributionProposed {
                    course_id,
                    kind,
                    comment,
                } => {
                    // serialize the contribution payload to JSONB
                    let contribution_json =
                        serde_json::to_value(kind).unwrap_or(serde_json::Value::Null);
                    sqlx::query(
                        "INSERT INTO contribution_list_view (aggregate_id, course_id, contribution, status, comment)
                         VALUES ($1, $2, $3::jsonb, 'Proposed', $4)
                         ON CONFLICT (aggregate_id) DO UPDATE
                         SET course_id = $2, contribution = $3::jsonb, status = 'Proposed', comment = $4",
                    )
                    .bind(aggregate_id)
                    .bind(course_id.to_string())
                    .bind(contribution_json)
                    .bind(comment)
                    .execute(&self.pool)
                    .await
                }
                ContributionEvent::ContributionApproved => sqlx::query(
                    "UPDATE contribution_list_view SET status = 'Approved' WHERE aggregate_id = $1",
                )
                .bind(aggregate_id)
                .execute(&self.pool)
                .await,
                ContributionEvent::ContributionDenied => sqlx::query(
                    "UPDATE contribution_list_view SET status = 'Denied' WHERE aggregate_id = $1",
                )
                .bind(aggregate_id)
                .execute(&self.pool)
                .await,
            };

            if let Err(e) = result {
                println!("ContributionListQuery error: {e}");
            }
        }
    }
}

pub struct ContributionProcessManager {
    pool: Pool<Postgres>,
    link: Arc<PostgresCqrs<Link>>,
    faq: Arc<PostgresCqrs<Faq>>,
    project_idea: Arc<PostgresCqrs<ProjectIdea>>,
}

enum AggregateCommand {
    Link(LinkCommand),
    Faq(FaqCommand),
    ProjectIdea(ProjectIdeaCommand),
}

impl AggregateCommand {
    fn id(&self) -> &Uuid {
        match self {
            AggregateCommand::Link(cmd) => cmd.id(),
            AggregateCommand::Faq(cmd) => cmd.id(),
            AggregateCommand::ProjectIdea(cmd) => cmd.id(),
        }
    }
}

impl ContributionProcessManager {
    pub fn new(
        pool: Pool<Postgres>,
        link: Arc<PostgresCqrs<Link>>,
        faq: Arc<PostgresCqrs<Faq>>,
        project_idea: Arc<PostgresCqrs<ProjectIdea>>,
    ) -> Self {
        Self {
            pool,
            link,
            faq,
            project_idea,
        }
    }

    async fn handle_approved(&self, contribution_id: &str) {
        // Fetch the stored contribution row (course_id + contribution JSONB)
        let row = match sqlx::query(
            "SELECT course_id, contribution FROM contribution_list_view WHERE aggregate_id = $1",
        )
        .bind(contribution_id)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(Some(row)) => row,
            Ok(None) => {
                println!(
                    "ContributionProcessManager: no contribution_list_view row for {contribution_id}"
                );
                return;
            }
            Err(e) => {
                println!("ContributionProcessManager: fetch error: {e}");
                return;
            }
        };

        let course_id: String = row.get("course_id");
        let contribution_json: serde_json::Value = row.get("contribution");

        // Deserialize back into your ContributionKind enum.
        // Adjust the type path to match where ContributionKind actually lives.
        let kind = match serde_json::from_value(contribution_json) {
            Ok(k) => k,
            Err(e) => {
                println!("ContributionProcessManager: failed to deserialize kind: {e}");
                return;
            }
        };

        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "contribution".to_string());
        let cmd = match kind {
            ContributionKind::Text(t) => match t {
                TextContributionKind::AddLink { label, url } => {
                    AggregateCommand::Link(LinkCommand::Create {
                        link_id: Uuid::new_v4(),
                        course_id: Uuid::parse_str(&course_id).unwrap(),
                        label,
                        url,
                    })
                }
                TextContributionKind::EditLink {
                    link_id,
                    label,
                    url,
                } => AggregateCommand::Link(LinkCommand::Update {
                    link_id,
                    course_id: Uuid::parse_str(&course_id).unwrap(),
                    label,
                    url,
                }),
                TextContributionKind::RemoveLink { link_id } => {
                    AggregateCommand::Link(LinkCommand::Delete {
                        link_id,
                        course_id: Uuid::parse_str(&course_id).unwrap(),
                    })
                }
                TextContributionKind::AddFaqEntry { question, answer } => {
                    AggregateCommand::Faq(FaqCommand::Create {
                        faq_id: Uuid::new_v4(),
                        course_id: Uuid::parse_str(&course_id).unwrap(),
                        question,
                        answer,
                    })
                }
                TextContributionKind::EditFaqEntry {
                    faq_id,
                    question,
                    answer,
                } => AggregateCommand::Faq(FaqCommand::Update {
                    faq_id,
                    course_id: Uuid::parse_str(&course_id).unwrap(),
                    question,
                    answer,
                }),
                TextContributionKind::RemoveFaqEntry { faq_id } => {
                    AggregateCommand::Faq(FaqCommand::Delete {
                        faq_id,
                        course_id: Uuid::parse_str(&course_id).unwrap(),
                    })
                }
                TextContributionKind::AddProjectIdea {
                    title,
                    body,
                    difficulty,
                } => AggregateCommand::ProjectIdea(ProjectIdeaCommand::Create {
                    idea_id: Uuid::new_v4(),
                    course_id: Uuid::parse_str(&course_id).unwrap(),
                    title,
                    body,
                    difficulty,
                }),
                TextContributionKind::EditProjectIdea {
                    idea_id,
                    title,
                    body,
                    difficulty,
                } => AggregateCommand::ProjectIdea(ProjectIdeaCommand::Update {
                    idea_id,
                    title,
                    body,
                    difficulty,
                }),
                TextContributionKind::RemoveProjectIdea { idea_id } => {
                    AggregateCommand::ProjectIdea(ProjectIdeaCommand::Delete { idea_id })
                }
            },
            _ => {
                todo!(
                    "ContributionProcessManager: contribution {contribution_id} has an unhandled kind, skipping"
                )
            }
        };

        let id = cmd.id().to_string();

        match cmd {
            AggregateCommand::Link(cmd) => {
                if let Err(e) = self.link.execute_with_metadata(&id, cmd, metadata).await {
                    println!(
                        "ContributionProcessManager: failed to execute link command for {contribution_id}: {e:?}"
                    );
                }
            }
            AggregateCommand::Faq(cmd) => {
                if let Err(e) = self.faq.execute_with_metadata(&id, cmd, metadata).await {
                    println!(
                        "ContributionProcessManager: failed to execute link command for {contribution_id}: {e:?}"
                    );
                }
            }
            AggregateCommand::ProjectIdea(cmd) => {
                if let Err(e) = self
                    .project_idea
                    .execute_with_metadata(&id, cmd, metadata)
                    .await
                {
                    println!(
                        "ContributionProcessManager: failed to execute project idea command for {contribution_id}: {e:?}"
                    );
                }
            }
        }
    }
}

#[async_trait]
impl Query<Contribution> for ContributionProcessManager {
    async fn dispatch(&self, _aggregate_id: &str, events: &[EventEnvelope<Contribution>]) {
        for event in events {
            match &event.payload {
                ContributionEvent::ContributionProposed {
                    course_id: _,
                    kind: _,
                    comment: _,
                } => (),
                ContributionEvent::ContributionApproved => {
                    let id = &event.aggregate_id;
                    self.handle_approved(id).await;
                }
                ContributionEvent::ContributionDenied => (),
            }
        }
    }
}
