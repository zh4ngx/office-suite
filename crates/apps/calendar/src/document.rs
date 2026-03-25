//! Calendar CRDT document.

use office_crdt::Document;
use office_core::DocumentId;
use crate::event::Event;

/// A calendar document containing events.
pub struct CalendarDocument {
    doc: Document,
}

impl CalendarDocument {
    pub fn new(id: DocumentId) -> Self {
        Self {
            doc: Document::new(id),
        }
    }

    /// Add an event to the calendar.
    pub fn add_event(&mut self, _event: Event) {
        // TODO: store event in CRDT
    }

    /// Get all events from the calendar.
    pub fn events(&self) -> Vec<Event> {
        // TODO: read events from CRDT
        Vec::new()
    }
}
