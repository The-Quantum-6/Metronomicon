use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ContributionCommand {
    /// Suggest a change
    ///
    /// Requires `contribute_file` for resource contributions and `contribute_text` for other contributions
    Propose {
        contribution_id: Uuid,
        course_id: Uuid,
        contribution: Contribution,
        comment: String,
    },

    /// Suggest a change
    ///
    /// Requires `moderate_file` for resource contributions and `moderate_text` for other contributions
    Moderate {
        contribution_id: Uuid,
        verdict: ModerationVerdict,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Contribution {
    File(FileContributionKind),
    Text(TextContributionKind),
}

pub type ContributionKind = Contribution;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum FileContributionKind {
    AddResource { title: String, key: Uuid },
    RemoveResource { resource_id: Uuid },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TextContributionKind {
    AddLink {
        label: String,
        url: String,
    },
    EditLink {
        link_id: Uuid,
        label: Option<String>,
        url: Option<String>,
    },
    RemoveLink {
        link_id: Uuid,
    },
    AddFaqEntry {
        question: String,
        answer: String,
    },
    EditFaqEntry {
        faq_id: Uuid,
        question: Option<String>,
        answer: Option<String>,
    },
    RemoveFaqEntry {
        faq_id: Uuid,
    },
    AddProjectIdea {
        title: String,
        body: String,
    },
    EditProjectIdea {
        idea_id: Uuid,
        title: Option<String>,
        body: Option<String>,
    },
    RemoveProjectIdea {
        idea_id: Uuid,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ModerationVerdict {
    Approve,
    Deny,
}

impl ContributionCommand {
    pub fn id(&self) -> Uuid {
        match self {
            ContributionCommand::Propose {
                contribution_id, ..
            } => *contribution_id,
            ContributionCommand::Moderate {
                contribution_id, ..
            } => *contribution_id,
        }
    }
}
