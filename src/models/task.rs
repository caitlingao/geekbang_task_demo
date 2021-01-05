use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: i32,
    pub content: String,
    pub finished: bool,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(content: &str, id: i32, user_id: i32) -> Self {
        Self {
            id,
            user_id,
            content: content.to_string(),
            finished: false,
            created_at: chrono::offset::Utc::now(),
            updated_at: chrono::offset::Utc::now(),
        }
    }
}
