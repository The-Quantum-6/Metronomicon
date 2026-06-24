use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The possible commands that may be issued against a course.
///
/// Each variant represents a single state transition. Authorization is
/// determined by the permission associated with the variant.
///
/// Superuser-only operations bypass the regular permission system.
#[derive(Debug, Serialize, Deserialize)]
pub enum CourseCommandPayload {
    /// Creates a new course.
    ///
    /// Permission: superuser only.
    Create {
        /// Human-readable course name.
        name: String,
        /// Course code, e.g. `DATA1700`.
        code: String,
        /// Academic field or discipline, e.g. `Computer Science`.
        field: String,
        /// Initial course description.
        description: String,
    },

    /// Permanently deletes the course.
    ///
    /// Permission: superuser only.
    Delete,

    /// Updates the course code.
    ///
    /// Permission: superuser only.
    UpdateCode {
        /// New course code.
        code: String,
    },

    /// Updates the course name.
    ///
    /// Permission: superuser only.
    UpdateName {
        /// New course name.
        name: String,
    },

    /// Updates the course description.
    ///
    /// Permission: `page_admin`.
    UpdateDescription {
        /// New description.
        description: String,
    },

    /// Updates the course's academic field.
    ///
    /// Permission: `page_admin`.
    UpdateField {
        /// New field value.
        field: String,
    },

    /// Adds a tag to the course.
    ///
    /// Permission: `page_admin`.
    AddTag {
        /// Tag to add.
        tag: String,
    },

    /// Removes a tag from the course.
    ///
    /// Permission: `page_admin`.
    RemoveTag {
        /// Tag to remove.
        tag: String,
    },

    /// Marks a link as officially endorsed by course maintainers.
    ///
    /// Permission: `page_admin`.
    MakeLinkOfficial {
        /// Identifier of the link.
        link_id: Uuid,
    },

    /// Removes official status from a link.
    ///
    /// Permission: `page_admin`.
    MakeLinkUnofficial {
        /// Identifier of the link.
        link_id: Uuid,
    },

    /// Marks a resource as officially endorsed by course maintainers.
    ///
    /// Permission: `page_admin`.
    MakeResourceOfficial {
        /// Identifier of the resource.
        resource_id: Uuid,
    },

    /// Removes official status from a resource.
    ///
    /// Permission: `page_admin`.
    MakeResourceUnofficial {
        /// Identifier of the resource.
        resource_id: Uuid,
    },

    // write_file
    /// Adds a resource directly.
    ///
    /// Permission: `write_file`.
    AddResource {
        /// Display title of the resource.
        title: String,
        /// Storage key referencing the uploaded file.
        key: Uuid,
    },

    /// Removes a resource directly.
    ///
    /// Permission: `write_file`.
    RemoveResource {
        /// Identifier of the resource.
        resource_id: Uuid,
    },

    /// Moderates a pending file suggestion.
    ///
    /// Permission: `moderate_file`.
    ModerateFile {
        /// Identifier of the suggestion being reviewed.
        suggestion_id: Uuid,
        /// Moderation outcome.
        verdict: ModerationVerdict,
        /// Type of suggestion being moderated.
        kind: FileSuggestionKind,
    },

    /// Proposes adding a resource.
    ///
    /// Permission: `suggest_file`.
    SuggestAddResource {
        /// Display title of the resource.
        title: String,
        /// Storage key referencing the uploaded file.
        key: Uuid,
    },

    /// Proposes removing a resource.
    ///
    /// Permission: `suggest_file`.
    SuggestRemoveResource {
        /// Identifier of the resource.
        resource_id: Uuid,
    },

    // write_text
    /// Creates a project idea.
    ///
    /// Permission: `write_text`.
    AddProjectIdea {
        /// Project title.
        title: String,
        /// Project description.
        body: String,
    },

    /// Updates a project idea.
    ///
    /// Permission: `write_text`.
    EditProjectIdea {
        /// Identifier of the project idea.
        idea_id: Uuid,
        /// Updated title.
        title: String,
        /// Updated description.
        body: String,
    },

    /// Removes a project idea.
    ///
    /// Permission: `write_text`.
    RemoveProjectIdea {
        /// Identifier of the project idea.
        idea_id: Uuid,
    },

    /// Creates a FAQ entry.
    ///
    /// Permission: `write_text`.
    AddFaqEntry {
        /// FAQ question.
        question: String,
        /// FAQ answer.
        answer: String,
    },

    /// Updates a FAQ entry.
    ///
    /// Permission: `write_text`.
    EditFaqEntry {
        /// Identifier of the FAQ entry.
        faq_id: Uuid,
        /// Updated answer.
        answer: String,
    },

    /// Removes a FAQ entry.
    ///
    /// Permission: `write_text`.
    RemoveFaqEntry {
        /// Identifier of the FAQ entry.
        faq_id: Uuid,
    },

    /// Creates a link.
    ///
    /// Permission: `write_text`.
    AddLink {
        /// Human-readable label.
        label: String,
        /// Destination URL.
        url: String,
    },

    /// Updates a link.
    ///
    /// Permission: `write_text`.
    EditLink {
        /// Identifier of the link.
        link_id: Uuid,
        /// Updated label.
        label: String,
        /// Updated URL.
        url: String,
    },

    /// Removes a link.
    ///
    /// Permission: `write_text`.
    RemoveLink {
        /// Identifier of the link.
        link_id: Uuid,
    },

    // moderate_text
    /// Moderates a pending text suggestion.
    ///
    /// Permission: `moderate_text`.
    ModerateText {
        /// Identifier of the suggestion being reviewed.
        suggestion_id: Uuid,
        /// Moderation outcome.
        verdict: ModerationVerdict,
        /// Type of suggestion being moderated.
        kind: TextSuggestionKind,
    },

    // suggest_text
    /// Proposes creating a project idea.
    ///
    /// Permission: `suggest_text`.
    SuggestAddProjectIdea {
        /// Project title.
        title: String,
        /// Project description.
        body: String,
    },

    /// Proposes modifying a project idea.
    ///
    /// Permission: `suggest_text`.
    SuggestEditProjectIdea {
        /// Identifier of the project idea.
        idea_id: Uuid,
        /// Proposed description.
        body: String,
    },

    /// Proposes removing a project idea.
    ///
    /// Permission: `suggest_text`.
    SuggestRemoveProjectIdea {
        /// Identifier of the project idea.
        idea_id: Uuid,
    },

    /// Proposes creating a FAQ entry.
    ///
    /// Permission: `suggest_text`.
    SuggestAddFaqEntry {
        /// FAQ question.
        question: String,
        /// FAQ answer.
        answer: String,
    },

    /// Proposes modifying a FAQ entry.
    ///
    /// Permission: `suggest_text`.
    SuggestEditFaqEntry {
        /// Identifier of the FAQ entry.
        faq_id: Uuid,
        /// Proposed answer.
        answer: String,
    },

    /// Proposes removing a FAQ entry.
    ///
    /// Permission: `suggest_text`.
    SuggestRemoveFaqEntry {
        /// Identifier of the FAQ entry.
        faq_id: Uuid,
    },

    /// Proposes creating a link.
    ///
    /// Permission: `suggest_text`.
    SuggestAddLink {
        /// Human-readable label.
        label: String,
        /// Destination URL.
        url: String,
    },

    /// Proposes modifying a link.
    ///
    /// Permission: `suggest_text`.
    SuggestEditLink {
        /// Identifier of the link.
        link_id: Uuid,
        /// Proposed label.
        label: String,
        /// Proposed URL.
        url: String,
    },

    /// Proposes removing a link.
    ///
    /// Permission: `suggest_text`.
    SuggestRemoveLink {
        /// Identifier of the link.
        link_id: Uuid,
    },
}

/// Outcome of a moderation decision.
#[derive(Debug, Serialize, Deserialize)]
pub enum ModerationVerdict {
    /// Accept the suggestion and apply it.
    Approve,

    /// Reject the suggestion without applying it.
    Deny,
}

/// The category of file suggestion being moderated.
#[derive(Debug, Serialize, Deserialize)]
pub enum FileSuggestionKind {
    /// Suggestion to create a resource.
    AddResource,

    /// Suggestion to remove a resource.
    RemoveResource,
}

/// The category of text suggestion being moderated.
#[derive(Debug, Serialize, Deserialize)]
pub enum TextSuggestionKind {
    AddProjectIdea,
    EditProjectIdea,
    RemoveProjectIdea,
    AddFaqEntry,
    EditFaqEntry,
    RemoveFaqEntry,
    AddLink,
    EditLink,
    RemoveLink,
}
