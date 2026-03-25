//! Notes CRDT document.

use office_crdt::Document;
use office_core::DocumentId;
use crate::note::Note;

/// A notes document containing notes.
pub struct NotesDocument {
    doc: Document,
}

impl NotesDocument {
    pub fn new(id: DocumentId) -> Self {
        Self {
            doc: Document::new(id),
        }
    }

    /// Add a note to the document.
    pub fn add_note(&mut self, _note: Note) {
        // TODO: store note in CRDT
    }

    /// Get all notes from the document.
    pub fn notes(&self) -> Vec<Note> {
        // TODO: read notes from CRDT
        Vec::new()
    }
}
