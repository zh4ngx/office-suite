//! Core type definitions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Unique identifier for a device in the sync mesh
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub String);

impl DeviceId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl Default for DeviceId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for an Automerge document
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(pub String);

/// Content identifier (SHA-256 hash, base58 encoded)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cid(pub String);

/// Timestamp wrapper with timezone awareness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamp {
    pub utc: DateTime<Utc>,
    pub timezone: Option<String>,
}

impl Timestamp {
    pub fn now() -> Self {
        Self {
            utc: Utc::now(),
            timezone: None,
        }
    }

    pub fn now_with_tz(timezone: String) -> Self {
        Self {
            utc: Utc::now(),
            timezone: Some(timezone),
        }
    }
}
