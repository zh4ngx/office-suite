//! Capability-based security for Wasm agents.

/// Capabilities granted to a Wasm agent.
#[derive(Debug, Clone, Default)]
pub struct Capabilities {
    /// Can access CRDT documents.
    pub crdt: bool,
    /// Can access network.
    pub network: bool,
    /// Can access storage.
    pub storage: bool,
}

impl Capabilities {
    /// No capabilities (most restricted).
    pub fn none() -> Self {
        Self::default()
    }

    /// Full capabilities (least restricted).
    pub fn full() -> Self {
        Self {
            crdt: true,
            network: true,
            storage: true,
        }
    }
}
