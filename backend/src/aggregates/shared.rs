use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize, Debug)]
pub enum Status {
    #[default]
    Uninitialized,
    Active,
    Deleted,
}
