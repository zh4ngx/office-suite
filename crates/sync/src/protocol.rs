//! Sync protocol definitions.

/// Protocol version.
pub const PROTOCOL_VERSION: &str = "0.1.0";

/// Message types for sync protocol.
#[derive(Debug, Clone)]
pub enum SyncMessage {
    /// Request document sync.
    SyncRequest { doc_id: Vec<u8> },
    /// Document update payload.
    Update { data: Vec<u8> },
}
