use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize, Debug, PartialEq)]
pub enum ReportStatus {
    #[default]
    Uninitialized,
    Open,
    Resolved,
    //Deleted,
}
