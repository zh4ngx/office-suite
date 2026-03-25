//! iCal (RFC 5545) serialization/deserialization.

use crate::event::Event;

/// Export events to iCal format.
pub fn to_ical(_events: &[Event]) -> String {
    // TODO: implement RFC 5545 serialization
    String::new()
}

/// Parse events from iCal format.
pub fn from_ical(_input: &str) -> Vec<Event> {
    // TODO: implement RFC 5545 parsing
    Vec::new()
}
