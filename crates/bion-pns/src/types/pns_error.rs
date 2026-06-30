//! PNS error surface — no infinite-db types leak to callers.

use core::fmt;

/// Errors returned by PNS gateway and transport operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PnsError {
    /// Target neuron was not found.
    NotFound,
    /// Revision conflict during optimistic write.
    RevisionConflict {
        /// Expected revision from the client.
        expected: u64,
        /// Actual revision in storage.
        actual: u64,
    },
    /// Background task failed or was cancelled.
    TaskJoin(String),
    /// Serialization or parsing failure.
    Codec(String),
    /// Transport I/O failure.
    Transport(String),
}

impl fmt::Display for PnsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "not found"),
            Self::RevisionConflict { expected, actual } => {
                write!(f, "revision conflict: expected {expected}, got {actual}")
            }
            Self::TaskJoin(msg) => write!(f, "task join: {msg}"),
            Self::Codec(msg) => write!(f, "codec: {msg}"),
            Self::Transport(msg) => write!(f, "transport: {msg}"),
        }
    }
}

impl std::error::Error for PnsError {}

impl From<tokio::task::JoinError> for PnsError {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::TaskJoin(err.to_string())
    }
}
