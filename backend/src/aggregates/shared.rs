use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize, Debug, PartialEq)]
pub enum Status {
    #[default]
    Uninitialized,
    Active,
    Deleted,
}
