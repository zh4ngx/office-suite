//! Content Identifier (CID) implementation.

use serde::{Deserialize, Serialize};

/// Content Identifier for content-addressed storage.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Cid(String);

impl Cid {
    /// Create CID from raw string.
    pub fn new(s: String) -> Self {
        Self(s)
    }

    /// Get the CID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Cid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
