use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize, Debug, PartialEq)]
pub enum Status {
    #[default]
    Uninitialized,
    Active,
    Deleted,
}
#[derive(Serialize, Default, Deserialize, Debug, PartialEq, Clone)]
pub enum Officiality {
    #[default]
    Unofficial,
    Official,
}