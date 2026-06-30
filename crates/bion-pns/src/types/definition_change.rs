//! Graph definition change notification.

use bion_soma::NeuronId;

/// Emitted when a subgraph definition changes in storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefinitionChange {
    /// Root of the changed subgraph.
    pub subgraph_id: NeuronId,
    /// Storage revision (wrapped from infinite-db `RevisionId`).
    pub revision: u64,
}
