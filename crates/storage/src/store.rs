//! Content-addressed storage backend.

use office_core::Cid;

/// Content-addressed storage for binary blobs.
pub struct ContentStore {
    // TODO: storage backend
}

impl ContentStore {
    pub fn new() -> Self {
        Self {}
    }

    /// Store content and return its CID.
    pub fn store(&self, _data: &[u8]) -> anyhow::Result<Cid> {
        // TODO: compute CID and store
        Ok(Cid("bafkqaaa".to_string())) // placeholder
    }

    /// Retrieve content by CID.
    pub fn get(&self, _cid: &Cid) -> Option<Vec<u8>> {
        // TODO: implement
        None
    }
}

impl Default for ContentStore {
    fn default() -> Self {
        Self::new()
    }
}
