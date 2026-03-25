//! Error types for the office suite.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OfficeError {
    #[error("CRDT error: {0}")]
    Crdt(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Sync error: {0}")]
    Sync(String),

    #[error("Sandbox error: {0}")]
    Sandbox(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, OfficeError>;
