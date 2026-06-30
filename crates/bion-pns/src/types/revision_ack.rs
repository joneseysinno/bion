//! Revision acknowledgement from a state write.

/// Returned by [`crate::gateway::PnsWriter::write_state`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevisionAck {
    /// New storage revision after the write.
    pub revision: u64,
}
