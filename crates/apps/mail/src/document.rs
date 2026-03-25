//! Mail CRDT document.

use office_crdt::Document;
use office_core::DocumentId;
use crate::thread::Thread;

/// A mail document containing threads.
pub struct MailDocument {
    doc: Document,
}

impl MailDocument {
    pub fn new(id: DocumentId) -> Self {
        Self {
            doc: Document::new(id),
        }
    }

    /// Add a thread to the mailbox.
    pub fn add_thread(&mut self, _thread: Thread) {
        // TODO: store thread in CRDT
    }

    /// Get all threads from the mailbox.
    pub fn threads(&self) -> Vec<Thread> {
        // TODO: read threads from CRDT
        Vec::new()
    }
}
