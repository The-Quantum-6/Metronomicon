use async_trait::async_trait;
use cqrs_es::{Aggregate, EventEnvelope, Query};

#[derive(Clone)]
pub struct SimpleLoggingQuery {}

#[async_trait]
impl<T> Query<T> for SimpleLoggingQuery
where
    T: Aggregate,
{
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<T>]) {
        for event in events {
            println!("{}-{}\n{:#?}", aggregate_id, event.sequence, event.payload);
        }
    }
}
