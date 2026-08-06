use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}
