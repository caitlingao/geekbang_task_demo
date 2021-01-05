use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct LoginInfo {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl LoginInfo {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
