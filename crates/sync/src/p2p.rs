//! P2P networking via libp2p.

/// P2P node for document sync.
pub struct P2pNode {
    // TODO: libp2p swarm
}

impl P2pNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for P2pNode {
    fn default() -> Self {
        Self::new()
    }
}
