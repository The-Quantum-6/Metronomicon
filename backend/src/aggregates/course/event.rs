use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CourseEvent {
    // Course lifecycle
    Created {
        name: String,
        code: String,
        field: String,
        description: String,
    },
    Deleted,

    // Metadata
    CodeUpdated {
        code: String,
    },
    NameUpdated {
        name: String,
    },
    DescriptionUpdated {
        description: String,
    },
    FieldUpdated {
        field: String,
    },

    // Tags
    TagAdded {
        tag: String,
    },
    TagRemoved {
        tag: String,
    },

    // Official status
    LinkMarkedOfficial {
        link_id: Uuid,
    },
    LinkMarkedUnofficial {
        link_id: Uuid,
    },
    ResourceMarkedOfficial {
        resource_id: Uuid,
    },
    ResourceMarkedUnofficial {
        resource_id: Uuid,
    },

    // Resources
    ResourceAdded {
        resource_id: Uuid,
        title: String,
        key: Uuid,
    },
    ResourceRemoved {
        resource_id: Uuid,
    },

    // File moderation
    FileSuggestionApproved {
        suggestion_id: Uuid,
    },
    FileSuggestionDenied {
        suggestion_id: Uuid,
    },

    // Resource suggestions
    ResourceSuggestionSubmitted {
        suggestion_id: Uuid,
        title: String,
        key: Uuid,
    },

    ResourceRemovalSuggestionSubmitted {
        suggestion_id: Uuid,
        resource_id: Uuid,
    },

    // Project ideas
    ProjectIdeaAdded {
        idea_id: Uuid,
        title: String,
        body: String,
    },
    ProjectIdeaEdited {
        idea_id: Uuid,
        title: String,
        body: String,
    },
    ProjectIdeaRemoved {
        idea_id: Uuid,
    },

    // FAQ
    FaqEntryAdded {
        faq_id: Uuid,
        question: String,
        answer: String,
    },
    FaqEntryEdited {
        faq_id: Uuid,
        answer: String,
    },
    FaqEntryRemoved {
        faq_id: Uuid,
    },

    // Links
    LinkAdded {
        link_id: Uuid,
        label: String,
        url: String,
    },
    LinkEdited {
        link_id: Uuid,
        label: String,
        url: String,
    },
    LinkRemoved {
        link_id: Uuid,
    },

    // Moderate text
    TextSuggestionApproved {
        suggestion_id: Uuid,
    },
    TextSuggestionDenied {
        suggestion_id: Uuid,
    },

    ProjectIdeaSuggestionSubmitted {
        suggestion_id: Uuid,
        title: String,
        body: String,
    },

    ProjectIdeaEditSuggestionSubmitted {
        suggestion_id: Uuid,
        idea_id: Uuid,
        body: String,
    },

    ProjectIdeaRemovalSuggestionSubmitted {
        suggestion_id: Uuid,
        idea_id: Uuid,
    },

    FaqEntryAddSuggestionSubmitted {
        suggestion_id: Uuid,
        question: String,
        answer: String,
    },

    FaqEntryEditSuggestionSubmitted {
        suggestion_id: Uuid,
        faq_id: Uuid,
        answer: String,
    },

    FaqEntryRemovalSuggestionSubmitted {
        suggestion_id: Uuid,
        faq_id: Uuid,
    },

    LinkAddSuggestionSubmitted {
        suggestion_id: Uuid,
        label: String,
        url: String,
    },

    LinkEditSuggestionSubmitted {
        suggestion_id: Uuid,
        link_id: Uuid,
        label: String,
        url: String,
    },

    LinkRemovalSuggestionSubmitted {
        suggestion_id: Uuid,
        link_id: Uuid,
    },
}

impl DomainEvent for CourseEvent {
    fn event_type(&self) -> String {
        match self {
            // Course lifecycle
            CourseEvent::Created { .. } => "CourseCreated",
            CourseEvent::Deleted => "CourseDeleted",

            // Metadata
            CourseEvent::CodeUpdated { .. } => "CourseCodeUpdated",
            CourseEvent::NameUpdated { .. } => "CourseNameUpdated",
            CourseEvent::DescriptionUpdated { .. } => "CourseDescriptionUpdated",
            CourseEvent::FieldUpdated { .. } => "CourseFieldUpdated",

            // Tags
            CourseEvent::TagAdded { .. } => "TagAdded",
            CourseEvent::TagRemoved { .. } => "TagRemoved",

            // Official status
            CourseEvent::LinkMarkedOfficial { .. } => "LinkMarkedOfficial",
            CourseEvent::LinkMarkedUnofficial { .. } => "LinkMarkedUnofficial",
            CourseEvent::ResourceMarkedOfficial { .. } => "ResourceMarkedOfficial",
            CourseEvent::ResourceMarkedUnofficial { .. } => "ResourceMarkedUnofficial",

            // Resources
            CourseEvent::ResourceAdded { .. } => "ResourceAdded",
            CourseEvent::ResourceRemoved { .. } => "ResourceRemoved",

            // File moderation
            CourseEvent::FileSuggestionApproved { .. } => "FileSuggestionApproved",
            CourseEvent::FileSuggestionDenied { .. } => "FileSuggestionDenied",

            // Resource suggestions
            CourseEvent::ResourceSuggestionSubmitted { .. } => "ResourceSuggestionSubmitted",
            CourseEvent::ResourceRemovalSuggestionSubmitted { .. } => {
                "ResourceRemovalSuggestionSubmitted"
            }

            // Project ideas
            CourseEvent::ProjectIdeaAdded { .. } => "ProjectIdeaAdded",
            CourseEvent::ProjectIdeaEdited { .. } => "ProjectIdeaEdited",
            CourseEvent::ProjectIdeaRemoved { .. } => "ProjectIdeaRemoved",

            // Project idea suggestions (NEW)
            CourseEvent::ProjectIdeaSuggestionSubmitted { .. } => "ProjectIdeaSuggestionSubmitted",
            CourseEvent::ProjectIdeaEditSuggestionSubmitted { .. } => {
                "ProjectIdeaEditSuggestionSubmitted"
            }
            CourseEvent::ProjectIdeaRemovalSuggestionSubmitted { .. } => {
                "ProjectIdeaRemovalSuggestionSubmitted"
            }

            // FAQ
            CourseEvent::FaqEntryAdded { .. } => "FaqEntryAdded",
            CourseEvent::FaqEntryEdited { .. } => "FaqEntryEdited",
            CourseEvent::FaqEntryRemoved { .. } => "FaqEntryRemoved",

            // FAQ suggestions (NEW)
            CourseEvent::FaqEntryAddSuggestionSubmitted { .. } => "FaqEntryAddSuggestionSubmitted",
            CourseEvent::FaqEntryEditSuggestionSubmitted { .. } => {
                "FaqEntryEditSuggestionSubmitted"
            }
            CourseEvent::FaqEntryRemovalSuggestionSubmitted { .. } => {
                "FaqEntryRemovalSuggestionSubmitted"
            }

            // Links
            CourseEvent::LinkAdded { .. } => "LinkAdded",
            CourseEvent::LinkEdited { .. } => "LinkEdited",
            CourseEvent::LinkRemoved { .. } => "LinkRemoved",

            // Link suggestions (NEW)
            CourseEvent::LinkAddSuggestionSubmitted { .. } => "LinkAddSuggestionSubmitted",
            CourseEvent::LinkEditSuggestionSubmitted { .. } => "LinkEditSuggestionSubmitted",
            CourseEvent::LinkRemovalSuggestionSubmitted { .. } => "LinkRemovalSuggestionSubmitted",

            // Text moderation
            CourseEvent::TextSuggestionApproved { .. } => "TextSuggestionApproved",
            CourseEvent::TextSuggestionDenied { .. } => "TextSuggestionDenied",
        }
        .to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
