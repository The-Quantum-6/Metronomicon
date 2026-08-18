use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query};
use postgres_es::PostgresCqrs;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use crate::aggregates::{
    contribution::{
        aggregate::Contribution,
        command::{ContributionKind, FileContributionKind, TextContributionKind},
        event::ContributionEvent,
    },
    faq::{aggregate::Faq, command::FaqCommand},
    link::{aggregate::Link, command::LinkCommand},
    project_idea::{aggregate::ProjectIdea, command::ProjectIdeaCommand},
    resource::{aggregate::Resource, command::ResourceCommand},
};
use crate::storage::Storage;

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
    resource: Arc<PostgresCqrs<Resource>>,
    storage: Storage,
}

enum AggregateCommand {
    Link(LinkCommand),
    Faq(FaqCommand),
    ProjectIdea(ProjectIdeaCommand),
    Resource(ResourceCommand),
}

impl AggregateCommand {
    fn id(&self) -> &Uuid {
        match self {
            AggregateCommand::Link(cmd) => cmd.id(),
            AggregateCommand::Faq(cmd) => cmd.id(),
            AggregateCommand::ProjectIdea(cmd) => cmd.id(),
            AggregateCommand::Resource(cmd) => cmd.id(),
        }
    }
}

impl ContributionProcessManager {
    pub fn new(
        pool: Pool<Postgres>,
        link: Arc<PostgresCqrs<Link>>,
        faq: Arc<PostgresCqrs<Faq>>,
        project_idea: Arc<PostgresCqrs<ProjectIdea>>,
        resource: Arc<PostgresCqrs<Resource>>,
        storage: Storage,
    ) -> Self {
        Self {
            pool,
            link,
            faq,
            project_idea,
            resource,
            storage,
        }
    }

    /// Reads back the projected contribution so a follow-up command can be
    /// built from it. Returns `None` (after logging) if the row is missing or
    /// unreadable, since a process manager has no caller to report errors to.
    async fn load_contribution(&self, contribution_id: &str) -> Option<(Uuid, ContributionKind)> {
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
                return None;
            }
            Err(e) => {
                println!("ContributionProcessManager: fetch error: {e}");
                return None;
            }
        };

        let course_id: String = row.get("course_id");
        let course_id = match Uuid::parse_str(&course_id) {
            Ok(id) => id,
            Err(e) => {
                println!(
                    "ContributionProcessManager: {contribution_id} has an unparseable course_id: {e}"
                );
                return None;
            }
        };

        let contribution_json: serde_json::Value = row.get("contribution");
        match serde_json::from_value(contribution_json) {
            Ok(kind) => Some((course_id, kind)),
            Err(e) => {
                println!("ContributionProcessManager: failed to deserialize kind: {e}");
                None
            }
        }
    }

    async fn handle_approved(&self, contribution_id: &str) {
        let Some((course_id, kind)) = self.load_contribution(contribution_id).await else {
            return;
        };

        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "contribution".to_string());
        let cmd = match kind {
            ContributionKind::Text(t) => match t {
                TextContributionKind::AddLink { label, url } => {
                    AggregateCommand::Link(LinkCommand::Create {
                        link_id: Uuid::new_v4(),
                        course_id,
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
                    course_id,
                    label,
                    url,
                }),
                TextContributionKind::RemoveLink { link_id } => {
                    AggregateCommand::Link(LinkCommand::Delete {
                        link_id,
                        course_id,
                    })
                }
                TextContributionKind::AddFaqEntry { question, answer } => {
                    AggregateCommand::Faq(FaqCommand::Create {
                        faq_id: Uuid::new_v4(),
                        course_id,
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
                    course_id,
                    question,
                    answer,
                }),
                TextContributionKind::RemoveFaqEntry { faq_id } => {
                    AggregateCommand::Faq(FaqCommand::Delete {
                        faq_id,
                        course_id,
                    })
                }
                TextContributionKind::AddProjectIdea {
                    title,
                    body,
                    difficulty,
                } => AggregateCommand::ProjectIdea(ProjectIdeaCommand::Create {
                    idea_id: Uuid::new_v4(),
                    course_id,
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
            ContributionKind::File(f) => match f {
                FileContributionKind::AddResource { title, key } => {
                    AggregateCommand::Resource(ResourceCommand::Create {
                        resource_id: Uuid::new_v4(),
                        course_id,
                        title,
                        key,
                    })
                }
                FileContributionKind::RemoveResource { resource_id } => {
                    AggregateCommand::Resource(ResourceCommand::Delete {
                        resource_id,
                        course_id,
                    })
                }
            },
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
            AggregateCommand::Resource(cmd) => {
                if let Err(e) = self.resource.execute_with_metadata(&id, cmd, metadata).await {
                    println!(
                        "ContributionProcessManager: failed to execute resource command for {contribution_id}: {e:?}"
                    );
                }
            }
        }
    }

    /// A denied file proposal leaves an orphaned object in Garage that nothing
    /// will ever point at, so drop it. Text proposals have nothing to clean up.
    async fn handle_denied(&self, contribution_id: &str) {
        let Some((_, kind)) = self.load_contribution(contribution_id).await else {
            return;
        };

        if let ContributionKind::File(FileContributionKind::AddResource { key, .. }) = kind {
            if let Err(e) = self.storage.delete(&key).await {
                println!(
                    "ContributionProcessManager: failed to delete orphaned object {key} for {contribution_id}: {e}"
                );
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
                ContributionEvent::ContributionDenied => {
                    let id = &event.aggregate_id;
                    self.handle_denied(id).await;
                }
            }
        }
    }
}
