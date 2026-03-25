//! Email message data model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An email message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub subject: String,
    pub from: String,
    pub to: Vec<String>,
    pub body: String,
    pub date: DateTime<Utc>,
}

impl Message {
    pub fn new(subject: String, from: String, to: Vec<String>, body: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            subject,
            from,
            to,
            body,
            date: Utc::now(),
        }
    }
}
