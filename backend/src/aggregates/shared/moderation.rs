use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ModerationVerdict {
    Approve,
    Deny,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SuggestionKind {
    File(FileSuggestionKind),
    Text(TextSuggestionKind),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FileSuggestionKind {
    AddResource,
    RemoveResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TextSuggestionKind {
    AddLink,
    EditLink,
    RemoveLink,
    AddFaqEntry,
    EditFaqEntry,
    RemoveFaqEntry,
    AddProjectIdea,
    EditProjectIdea,
    RemoveProjectIdea,
}
