use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum SuggestionCommand {
    /// Suggest a change
    ///
    /// Requires `suggest_file` for resource suggestions and `suggest_text` for other suggestions
    Propose {
        course_id: Uuid,
        suggestion: Suggestion,
    },

    /// Suggest a change
    ///
    /// Requires `moderate_file` for resource suggestions and `moderate_text` for other suggestions
    Moderate {
        suggestion_id: Uuid,
        verdict: ModerationVerdict,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Suggestion {
    File(FileSuggestionKind),
    Text(TextSuggestionKind),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum FileSuggestionKind {
    AddResource {
        course_id: Uuid,
        title: String,
        key: Uuid,
    },
    RemoveResource {
        resource_id: Uuid,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TextSuggestionKind {
    AddLink {
        course_id: Uuid,
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
        course_id: Uuid,
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
        course_id: Uuid,
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
