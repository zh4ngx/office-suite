//! Sync state tracking for CRDT documents.

use office_core::DocumentId;

/// Tracks sync state between peers.
pub struct SyncState {
    doc_id: DocumentId,
    last_sync_seq: u64,
}

impl SyncState {
    pub fn new(doc_id: DocumentId) -> Self {
        Self {
            doc_id,
            last_sync_seq: 0,
        }
    }
}
