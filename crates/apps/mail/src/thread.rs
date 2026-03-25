//! Email thread (conversation) model.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::message::Message;

/// An email thread (conversation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    pub id: Uuid,
    pub messages: Vec<Message>,
    pub subject: String,
}

impl Thread {
    pub fn new(subject: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            messages: Vec::new(),
            subject,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}
