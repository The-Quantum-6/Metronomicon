use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::aggregates::project_idea::difficulty::Difficulty;

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

/// The kind of contribution, containing the relevant data.
/// Either Text or File type, further divided.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Contribution {
    /// Represents contributions that may include file uploads, like resources on courses.
    /// This is seperate from Text suggestions due to permission management. File uploads are considered to be of higher risk than text changes.
    File(FileContributionKind),
    /// Represents most contributions, including info, links, project ideas, etc.
    Text(TextContributionKind),
}

/// Rename for export, better clarity in some modules
pub type ContributionKind = Contribution;

/// The only file contribution currently available are course resources.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum FileContributionKind {
    /// Key refers to the objects key in S3(Garage)
    AddResource {
        title: String,
        key: String,
    },
    RemoveResource {
        resource_id: Uuid,
    },
}

/// Most contributions are text contributions. Text contributions are considered lower risk than file contributions and are thus seperated for permission management.
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
        difficulty: Difficulty,
    },
    EditProjectIdea {
        idea_id: Uuid,
        title: Option<String>,
        body: Option<String>,
        difficulty: Option<Difficulty>,
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
                contribution_id,
                course_id: _,
                contribution: _,
                comment: _,
            } => *contribution_id,
            ContributionCommand::Moderate {
                contribution_id,
                verdict: _,
            } => *contribution_id,
        }
    }
}
