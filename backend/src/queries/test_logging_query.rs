use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query};

use crate::aggregates::course::aggregate::Course;

pub struct SimpleLoggingQuery {}

#[async_trait]
impl Query<Course> for SimpleLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Course>]) {
        for event in events {
            println!("{}-{}\n{:#?}", aggregate_id, event.sequence, &event.payload);
        }
    }
}
