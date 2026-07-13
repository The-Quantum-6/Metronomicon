use async_trait::async_trait;
use cqrs_es::persist::ViewRepository;
use cqrs_es::{EventEnvelope, Query};
use std::sync::Arc;
use uuid::Uuid;

use crate::aggregates::shared::{Officiality, Status};
use crate::views::faq::FaqDetailView;
use crate::{
    aggregates::faq::{aggregate::Faq, event::FaqEvent},
    views::course::active_detailed::CourseDetailViewRepo,
};

pub struct CourseFaqQuery {
    view_repo: Arc<CourseDetailViewRepo>,
}

impl CourseFaqQuery {
    pub fn new(view_repo: Arc<CourseDetailViewRepo>) -> Self {
        Self { view_repo }
    }
}

#[async_trait]
impl Query<Faq> for CourseFaqQuery {
    async fn dispatch(&self, faq_id: &str, events: &[EventEnvelope<Faq>]) {
        for event in events {
            let course_id = match &event.payload {
                FaqEvent::FaqCreated { course_id, .. } => course_id.to_string(),
                FaqEvent::FaqUpdated { course_id, .. } => course_id.to_string(),
                FaqEvent::FaqDeleted { course_id, .. } => course_id.to_string(),
                FaqEvent::FaqOfficialStatusChanged { course_id, .. } => course_id.to_string(),
            };

            let (mut view, context) = match self.view_repo.load_with_context(&course_id).await {
                Ok(Some(vc)) => vc,
                Ok(None) => {
                    println!("CourseFaqQuery: no course view found for {course_id}");
                    continue;
                }
                Err(e) => {
                    println!("CourseFaqQuery load error: {e}");
                    continue;
                }
            };

            let faq_id = Uuid::parse_str(faq_id).unwrap();

            match &event.payload {
                FaqEvent::FaqCreated {
                    course_id,
                    question,
                    answer,
                    ..
                } => {
                    view.faqs.push(FaqDetailView {
                        faq_id,
                        status: Status::Active,
                        officiality: Officiality::Unofficial,
                        course_id: *course_id,
                        question: question.clone(),
                        answer: answer.clone(),
                    });
                }
                FaqEvent::FaqUpdated { question, answer, .. } => {
                    let l = view
                        .faqs
                        .iter_mut()
                        .find(|l| l.faq_id == faq_id)
                        .unwrap();
                    if let Some(question) = question {
                        l.question = question.clone();
                    }
                    if let Some(answer) = answer {
                        l.answer = answer.clone();
                    }
                }
                FaqEvent::FaqDeleted { .. } => {
                    view.faqs.retain(|l| l.faq_id != faq_id);
                }
                FaqEvent::FaqOfficialStatusChanged { officiality, .. } => {
                    let l = view
                        .faqs
                        .iter_mut()
                        .find(|l| l.faq_id == faq_id)
                        .unwrap();
                    l.officiality = officiality.clone();
                }
            }

            // Save back under the course_id key
            if let Err(e) = self.view_repo.update_view(view, context).await {
                println!("CourseFaqQuery save error: {e}");
            }
        }
    }
}
