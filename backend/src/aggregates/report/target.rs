use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Default, Deserialize, Debug, PartialEq, Clone)]
pub enum ReportTarget {
    #[default]
    Site,
    Course(Uuid),
}
