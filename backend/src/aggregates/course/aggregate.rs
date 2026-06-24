use crate::aggregates::course::command::CourseCommandPayload;
use crate::aggregates::course::error::CourseError;
use crate::aggregates::course::event::CourseEvent;
use crate::aggregates::course::services::CourseServices;
use cqrs_es::{Aggregate, event_sink::EventSink};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::future::Future;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Course {
    // Identity
    pub id: Uuid,

    // Metadata
    pub name: String,
    pub code: String,
    pub field: String,
    pub description: String,

    // Tags
    pub tags: HashSet<String>,

    // Resources
    pub resources: HashMap<Uuid, Resource>,

    // Project ideas
    pub project_ideas: HashMap<Uuid, ProjectIdea>,

    // FAQ
    pub faq_entries: HashMap<Uuid, FaqEntry>,

    // Links
    pub links: HashMap<Uuid, Link>,

    // Suggestions
    pub suggestions: HashMap<Uuid, Suggestion>,

    // Lifecycle
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: Uuid,
    pub title: String,
    pub key: Uuid,
    pub official: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectIdea {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqEntry {
    pub id: Uuid,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: Uuid,
    pub label: String,
    pub url: String,
    pub official: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Suggestion {
    ResourceAddition {
        id: Uuid,
        title: String,
        key: Uuid,
    },
    ResourceRemoval {
        id: Uuid,
        resource_id: Uuid,
    },
    ProjectIdeaAddition {
        id: Uuid,
        title: String,
        body: String,
    },
    ProjectIdeaEdit {
        id: Uuid,
        idea_id: Uuid,
        body: String,
    },
    ProjectIdeaRemoval {
        id: Uuid,
        idea_id: Uuid,
    },
    FaqAdd {
        id: Uuid,
        question: String,
        answer: String,
    },
    FaqEdit {
        id: Uuid,
        faq_id: Uuid,
        answer: String,
    },
    FaqRemoval {
        id: Uuid,
        faq_id: Uuid,
    },
    LinkAdd {
        id: Uuid,
        label: String,
        url: String,
    },
    LinkEdit {
        id: Uuid,
        link_id: Uuid,
        label: String,
        url: String,
    },
    LinkRemoval {
        id: Uuid,
        link_id: Uuid,
    },
}

impl Default for Suggestion {
    fn default() -> Self {
        Suggestion::ResourceAddition {
            id: Uuid::nil(),
            title: String::new(),
            key: Uuid::nil(),
        }
    }
}

impl Aggregate for Course {
    const TYPE: &'static str = "course";
    type Command = CourseCommandPayload;
    type Event = CourseEvent;
    type Error = CourseError;
    type Services = CourseServices;

    fn handle(
        &mut self,
        command: Self::Command,
        _services: &Self::Services,
        sink: &EventSink<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            use CourseCommandPayload::*;

            match command {
                Create {
                    name,
                    code,
                    field,
                    description,
                } => {
                    sink.write(
                        CourseEvent::Created {
                            name,
                            code,
                            field,
                            description,
                        },
                        self,
                    )
                    .await;
                }
                Delete => {
                    sink.write(CourseEvent::Deleted, self).await;
                }
                UpdateCode { code } => {
                    sink.write(CourseEvent::CodeUpdated { code }, self).await;
                }
                UpdateName { name } => {
                    sink.write(CourseEvent::NameUpdated { name }, self).await;
                }
                UpdateDescription { description } => {
                    sink.write(CourseEvent::DescriptionUpdated { description }, self)
                        .await;
                }
                UpdateField { field } => {
                    sink.write(CourseEvent::FieldUpdated { field }, self).await;
                }
                AddTag { tag } => {
                    sink.write(CourseEvent::TagAdded { tag }, self).await;
                }
                RemoveTag { tag } => {
                    sink.write(CourseEvent::TagRemoved { tag }, self).await;
                }
                MakeLinkOfficial { link_id } => {
                    sink.write(CourseEvent::LinkMarkedOfficial { link_id }, self)
                        .await;
                }
                MakeLinkUnofficial { link_id } => {
                    sink.write(CourseEvent::LinkMarkedUnofficial { link_id }, self)
                        .await;
                }
                MakeResourceOfficial { resource_id } => {
                    sink.write(CourseEvent::ResourceMarkedOfficial { resource_id }, self)
                        .await;
                }
                MakeResourceUnofficial { resource_id } => {
                    sink.write(CourseEvent::ResourceMarkedUnofficial { resource_id }, self)
                        .await;
                }
                AddResource { title, key } => {
                    let resource_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::ResourceAdded {
                            resource_id,
                            title,
                            key,
                        },
                        self,
                    )
                    .await;
                }
                RemoveResource { resource_id } => {
                    sink.write(CourseEvent::ResourceRemoved { resource_id }, self)
                        .await;
                }
                ModerateFile {
                    suggestion_id,
                    verdict,
                    kind: _,
                } => match verdict {
                    crate::aggregates::course::command::ModerationVerdict::Approve => {
                        sink.write(CourseEvent::FileSuggestionApproved { suggestion_id }, self)
                            .await;
                    }
                    crate::aggregates::course::command::ModerationVerdict::Deny => {
                        sink.write(CourseEvent::FileSuggestionDenied { suggestion_id }, self)
                            .await;
                    }
                },
                SuggestAddResource { title, key } => {
                    let suggestion_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::ResourceSuggestionSubmitted {
                            suggestion_id,
                            title,
                            key,
                        },
                        self,
                    )
                    .await;
                }
                SuggestRemoveResource { resource_id } => {
                    let suggestion_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::ResourceRemovalSuggestionSubmitted {
                            suggestion_id,
                            resource_id,
                        },
                        self,
                    )
                    .await;
                }
                AddProjectIdea { title, body } => {
                    let idea_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::ProjectIdeaAdded {
                            idea_id,
                            title,
                            body,
                        },
                        self,
                    )
                    .await;
                }
                EditProjectIdea {
                    idea_id,
                    title,
                    body,
                } => {
                    sink.write(
                        CourseEvent::ProjectIdeaEdited {
                            idea_id,
                            title,
                            body,
                        },
                        self,
                    )
                    .await;
                }
                RemoveProjectIdea { idea_id } => {
                    sink.write(CourseEvent::ProjectIdeaRemoved { idea_id }, self)
                        .await;
                }
                AddFaqEntry { question, answer } => {
                    let faq_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::FaqEntryAdded {
                            faq_id,
                            question,
                            answer,
                        },
                        self,
                    )
                    .await;
                }
                EditFaqEntry { faq_id, answer } => {
                    sink.write(CourseEvent::FaqEntryEdited { faq_id, answer }, self)
                        .await;
                }
                RemoveFaqEntry { faq_id } => {
                    sink.write(CourseEvent::FaqEntryRemoved { faq_id }, self)
                        .await;
                }
                AddLink { label, url } => {
                    let link_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::LinkAdded {
                            link_id,
                            label,
                            url,
                        },
                        self,
                    )
                    .await;
                }
                EditLink {
                    link_id,
                    label,
                    url,
                } => {
                    sink.write(
                        CourseEvent::LinkEdited {
                            link_id,
                            label,
                            url,
                        },
                        self,
                    )
                    .await;
                }
                RemoveLink { link_id } => {
                    sink.write(CourseEvent::LinkRemoved { link_id }, self).await;
                }
                ModerateText {
                    suggestion_id,
                    verdict,
                    kind: _,
                } => match verdict {
                    crate::aggregates::course::command::ModerationVerdict::Approve => {
                        sink.write(CourseEvent::TextSuggestionApproved { suggestion_id }, self)
                            .await;
                    }
                    crate::aggregates::course::command::ModerationVerdict::Deny => {
                        sink.write(CourseEvent::TextSuggestionDenied { suggestion_id }, self)
                            .await;
                    }
                },
                SuggestAddProjectIdea { title, body } => {
                    let suggestion_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::ProjectIdeaSuggestionSubmitted {
                            suggestion_id,
                            title,
                            body,
                        },
                        self,
                    )
                    .await;
                }
                SuggestEditProjectIdea { idea_id, body } => {
                    let suggestion_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::ProjectIdeaEditSuggestionSubmitted {
                            suggestion_id,
                            idea_id,
                            body,
                        },
                        self,
                    )
                    .await;
                }
                SuggestRemoveProjectIdea { idea_id } => {
                    let suggestion_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::ProjectIdeaRemovalSuggestionSubmitted {
                            suggestion_id,
                            idea_id,
                        },
                        self,
                    )
                    .await;
                }
                SuggestAddFaqEntry { question, answer } => {
                    let suggestion_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::FaqEntryAddSuggestionSubmitted {
                            suggestion_id,
                            question,
                            answer,
                        },
                        self,
                    )
                    .await;
                }
                SuggestEditFaqEntry { faq_id, answer } => {
                    let suggestion_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::FaqEntryEditSuggestionSubmitted {
                            suggestion_id,
                            faq_id,
                            answer,
                        },
                        self,
                    )
                    .await;
                }
                SuggestRemoveFaqEntry { faq_id } => {
                    let suggestion_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::FaqEntryRemovalSuggestionSubmitted {
                            suggestion_id,
                            faq_id,
                        },
                        self,
                    )
                    .await;
                }
                SuggestAddLink { label, url } => {
                    let suggestion_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::LinkAddSuggestionSubmitted {
                            suggestion_id,
                            label,
                            url,
                        },
                        self,
                    )
                    .await;
                }
                SuggestEditLink {
                    link_id,
                    label,
                    url,
                } => {
                    let suggestion_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::LinkEditSuggestionSubmitted {
                            suggestion_id,
                            link_id,
                            label,
                            url,
                        },
                        self,
                    )
                    .await;
                }
                SuggestRemoveLink { link_id } => {
                    let suggestion_id = Uuid::new_v4();
                    sink.write(
                        CourseEvent::LinkRemovalSuggestionSubmitted {
                            suggestion_id,
                            link_id,
                        },
                        self,
                    )
                    .await;
                }
            }

            Ok(())
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            // Lifecycle
            CourseEvent::Created {
                name,
                code,
                field,
                description,
            } => {
                self.name = name;
                self.code = code;
                self.field = field;
                self.description = description;
                self.deleted = false;
            }
            CourseEvent::Deleted => {
                self.deleted = true;
            }

            // Metadata
            CourseEvent::CodeUpdated { code } => self.code = code,
            CourseEvent::NameUpdated { name } => self.name = name,
            CourseEvent::DescriptionUpdated { description } => self.description = description,
            CourseEvent::FieldUpdated { field } => self.field = field,

            // Tags
            CourseEvent::TagAdded { tag } => {
                self.tags.insert(tag);
            }
            CourseEvent::TagRemoved { tag } => {
                self.tags.remove(&tag);
            }

            // Official status
            CourseEvent::LinkMarkedOfficial { link_id } => {
                if let Some(link) = self.links.get_mut(&link_id) {
                    link.official = true;
                }
            }
            CourseEvent::LinkMarkedUnofficial { link_id } => {
                if let Some(link) = self.links.get_mut(&link_id) {
                    link.official = false;
                }
            }
            CourseEvent::ResourceMarkedOfficial { resource_id } => {
                if let Some(res) = self.resources.get_mut(&resource_id) {
                    res.official = true;
                }
            }
            CourseEvent::ResourceMarkedUnofficial { resource_id } => {
                if let Some(res) = self.resources.get_mut(&resource_id) {
                    res.official = false;
                }
            }

            // Resources
            CourseEvent::ResourceAdded {
                resource_id,
                title,
                key,
            } => {
                let res = Resource {
                    id: resource_id,
                    title,
                    key,
                    official: false,
                };
                self.resources.insert(resource_id, res);
            }
            CourseEvent::ResourceRemoved { resource_id } => {
                self.resources.remove(&resource_id);
            }

            // File moderation
            CourseEvent::FileSuggestionApproved { suggestion_id }
            | CourseEvent::FileSuggestionDenied { suggestion_id } => {
                self.suggestions.remove(&suggestion_id);
            }

            // Resource suggestions
            CourseEvent::ResourceSuggestionSubmitted {
                suggestion_id,
                title,
                key,
            } => {
                self.suggestions.insert(
                    suggestion_id,
                    Suggestion::ResourceAddition {
                        id: suggestion_id,
                        title,
                        key,
                    },
                );
            }
            CourseEvent::ResourceRemovalSuggestionSubmitted {
                suggestion_id,
                resource_id,
            } => {
                self.suggestions.insert(
                    suggestion_id,
                    Suggestion::ResourceRemoval {
                        id: suggestion_id,
                        resource_id,
                    },
                );
            }

            // Project ideas
            CourseEvent::ProjectIdeaAdded {
                idea_id,
                title,
                body,
            } => {
                self.project_ideas.insert(
                    idea_id,
                    ProjectIdea {
                        id: idea_id,
                        title,
                        body,
                    },
                );
            }
            CourseEvent::ProjectIdeaEdited {
                idea_id,
                title,
                body,
            } => {
                if let Some(idea) = self.project_ideas.get_mut(&idea_id) {
                    idea.title = title;
                    idea.body = body;
                }
            }
            CourseEvent::ProjectIdeaRemoved { idea_id } => {
                self.project_ideas.remove(&idea_id);
            }

            // Project idea suggestions
            CourseEvent::ProjectIdeaSuggestionSubmitted {
                suggestion_id,
                title,
                body,
            } => {
                self.suggestions.insert(
                    suggestion_id,
                    Suggestion::ProjectIdeaAddition {
                        id: suggestion_id,
                        title,
                        body,
                    },
                );
            }
            CourseEvent::ProjectIdeaEditSuggestionSubmitted {
                suggestion_id,
                idea_id,
                body,
            } => {
                self.suggestions.insert(
                    suggestion_id,
                    Suggestion::ProjectIdeaEdit {
                        id: suggestion_id,
                        idea_id,
                        body,
                    },
                );
            }
            CourseEvent::ProjectIdeaRemovalSuggestionSubmitted {
                suggestion_id,
                idea_id,
            } => {
                self.suggestions.insert(
                    suggestion_id,
                    Suggestion::ProjectIdeaRemoval {
                        id: suggestion_id,
                        idea_id,
                    },
                );
            }

            // FAQ
            CourseEvent::FaqEntryAdded {
                faq_id,
                question,
                answer,
            } => {
                self.faq_entries.insert(
                    faq_id,
                    FaqEntry {
                        id: faq_id,
                        question,
                        answer,
                    },
                );
            }
            CourseEvent::FaqEntryEdited { faq_id, answer } => {
                if let Some(faq) = self.faq_entries.get_mut(&faq_id) {
                    faq.answer = answer;
                }
            }
            CourseEvent::FaqEntryRemoved { faq_id } => {
                self.faq_entries.remove(&faq_id);
            }

            // FAQ suggestions
            CourseEvent::FaqEntryAddSuggestionSubmitted {
                suggestion_id,
                question,
                answer,
            } => {
                self.suggestions.insert(
                    suggestion_id,
                    Suggestion::FaqAdd {
                        id: suggestion_id,
                        question,
                        answer,
                    },
                );
            }
            CourseEvent::FaqEntryEditSuggestionSubmitted {
                suggestion_id,
                faq_id,
                answer,
            } => {
                self.suggestions.insert(
                    suggestion_id,
                    Suggestion::FaqEdit {
                        id: suggestion_id,
                        faq_id,
                        answer,
                    },
                );
            }
            CourseEvent::FaqEntryRemovalSuggestionSubmitted {
                suggestion_id,
                faq_id,
            } => {
                self.suggestions.insert(
                    suggestion_id,
                    Suggestion::FaqRemoval {
                        id: suggestion_id,
                        faq_id,
                    },
                );
            }

            // Links
            CourseEvent::LinkAdded {
                link_id,
                label,
                url,
            } => {
                self.links.insert(
                    link_id,
                    Link {
                        id: link_id,
                        label,
                        url,
                        official: false,
                    },
                );
            }
            CourseEvent::LinkEdited {
                link_id,
                label,
                url,
            } => {
                if let Some(link) = self.links.get_mut(&link_id) {
                    link.label = label;
                    link.url = url;
                }
            }
            CourseEvent::LinkRemoved { link_id } => {
                self.links.remove(&link_id);
            }

            // Link suggestions
            CourseEvent::LinkAddSuggestionSubmitted {
                suggestion_id,
                label,
                url,
            } => {
                self.suggestions.insert(
                    suggestion_id,
                    Suggestion::LinkAdd {
                        id: suggestion_id,
                        label,
                        url,
                    },
                );
            }
            CourseEvent::LinkEditSuggestionSubmitted {
                suggestion_id,
                link_id,
                label,
                url,
            } => {
                self.suggestions.insert(
                    suggestion_id,
                    Suggestion::LinkEdit {
                        id: suggestion_id,
                        link_id,
                        label,
                        url,
                    },
                );
            }
            CourseEvent::LinkRemovalSuggestionSubmitted {
                suggestion_id,
                link_id,
            } => {
                self.suggestions.insert(
                    suggestion_id,
                    Suggestion::LinkRemoval {
                        id: suggestion_id,
                        link_id,
                    },
                );
            }

            // Text moderation
            CourseEvent::TextSuggestionApproved { suggestion_id }
            | CourseEvent::TextSuggestionDenied { suggestion_id } => {
                self.suggestions.remove(&suggestion_id);
            }
        }
    }
}

#[cfg(test)]
mod aggregate_tests {
    use super::Course;
    use crate::aggregates::course::command::CourseCommandPayload;
    use crate::aggregates::course::event::CourseEvent;
    use crate::aggregates::course::services::CourseServices;
    use cqrs_es::test::TestFramework;
    use uuid::Uuid;

    type CourseTestFramework = TestFramework<Course>;

    #[test]
    fn test_create_course() {
        let command = CourseCommandPayload::Create {
            name: "Algorithms".to_string(),
            code: "DATA1700".to_string(),
            field: "Computer Science".to_string(),
            description: "Introductory algorithms course".to_string(),
        };

        CourseTestFramework::with(CourseServices)
            .given_no_previous_events()
            .when(command)
            .then_expect_events(vec![CourseEvent::Created {
                name: "Algorithms".to_string(),
                code: "DATA1700".to_string(),
                field: "Computer Science".to_string(),
                description: "Introductory algorithms course".to_string(),
            }]);
    }

    #[test]
    fn test_update_name() {
        let previous = CourseEvent::Created {
            name: "Algorithms".to_string(),
            code: "DATA1700".to_string(),
            field: "Computer Science".to_string(),
            description: "Introductory algorithms course".to_string(),
        };
        let command = CourseCommandPayload::UpdateName {
            name: "Advanced Algorithms".to_string(),
        };

        CourseTestFramework::with(CourseServices)
            .given(vec![previous])
            .when(command)
            .then_expect_events(vec![CourseEvent::NameUpdated {
                name: "Advanced Algorithms".to_string(),
            }]);
    }

    #[test]
    fn test_add_and_remove_tag() {
        let previous = CourseEvent::Created {
            name: "Algorithms".to_string(),
            code: "DATA1700".to_string(),
            field: "Computer Science".to_string(),
            description: "Introductory algorithms course".to_string(),
        };

        CourseTestFramework::with(CourseServices)
            .given(vec![previous.clone()])
            .when(CourseCommandPayload::AddTag {
                tag: "exam-prep".to_string(),
            })
            .then_expect_events(vec![CourseEvent::TagAdded {
                tag: "exam-prep".to_string(),
            }]);

        CourseTestFramework::with(CourseServices)
            .given(vec![previous])
            .when(CourseCommandPayload::RemoveTag {
                tag: "exam-prep".to_string(),
            })
            .then_expect_events(vec![CourseEvent::TagRemoved {
                tag: "exam-prep".to_string(),
            }]);
    }

    #[test]
    fn test_add_resource_generates_identifier() {
        let previous = CourseEvent::Created {
            name: "Algorithms".to_string(),
            code: "DATA1700".to_string(),
            field: "Computer Science".to_string(),
            description: "Introductory algorithms course".to_string(),
        };

        let result = CourseTestFramework::with(CourseServices)
            .given(vec![previous])
            .when(CourseCommandPayload::AddResource {
                title: "Lecture notes".to_string(),
                key: Uuid::new_v4(),
            })
            .inspect_result()
            .expect("expected add resource to succeed");

        match result.as_slice() {
            [CourseEvent::ResourceAdded {
                resource_id,
                title,
                key,
            }] => {
                assert_ne!(*resource_id, Uuid::nil());
                assert_eq!(title, "Lecture notes");
                assert_ne!(*key, Uuid::nil());
            }
            other => panic!("unexpected events: {other:?}"),
        }
    }

    #[test]
    fn test_suggest_link_addition_generates_suggestion_id() {
        let previous = CourseEvent::Created {
            name: "Algorithms".to_string(),
            code: "DATA1700".to_string(),
            field: "Computer Science".to_string(),
            description: "Introductory algorithms course".to_string(),
        };

        let result = CourseTestFramework::with(CourseServices)
            .given(vec![previous])
            .when(CourseCommandPayload::SuggestAddLink {
                label: "Course homepage".to_string(),
                url: "https://example.com/course".to_string(),
            })
            .inspect_result()
            .expect("expected link suggestion to succeed");

        match result.as_slice() {
            [CourseEvent::LinkAddSuggestionSubmitted {
                suggestion_id,
                label,
                url,
            }] => {
                assert_ne!(*suggestion_id, Uuid::nil());
                assert_eq!(label, "Course homepage");
                assert_eq!(url, "https://example.com/course");
            }
            other => panic!("unexpected events: {other:?}"),
        }
    }
}
