use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct test {
    pub id: i32,
    pub content: String,
}

#[derive(Deserialize)]
pub struct newTestContent {
    pub content: String,
}

