//! NATS relay for fallback sync when P2P unavailable.

/// NATS relay client.
pub struct NatsRelay {
    // TODO: async-nats client
}

impl NatsRelay {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for NatsRelay {
    fn default() -> Self {
        Self::new()
    }
}
