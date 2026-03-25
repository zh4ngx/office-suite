//! CRDT document abstraction over Automerge.

use automerge::Automerge;
use office_core::DocumentId;

/// A CRDT document that can be synced across devices.
pub struct Document {
    id: DocumentId,
    doc: Automerge,
}

impl Document {
    pub fn new(id: DocumentId) -> Self {
        Self {
            id,
            doc: Automerge::new(),
        }
    }

    /// Get the document ID.
    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    /// Encode document for storage/transfer.
    pub fn encode(&self) -> Vec<u8> {
        self.doc.save()
    }
}
