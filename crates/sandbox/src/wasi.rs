//! Custom WASI interfaces for CRDT and agent operations.

/// Placeholder for wasi:crdt interface implementation.
/// See ARCHITECTURE.md for interface design.
pub struct WasiCrdtHost {
    // TODO: implement wasi:crdt host functions
}

impl WasiCrdtHost {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for WasiCrdtHost {
    fn default() -> Self {
        Self::new()
    }
}
