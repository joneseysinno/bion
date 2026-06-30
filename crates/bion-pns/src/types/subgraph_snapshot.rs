//! Immutable snapshot of a subgraph definition.

use bion_soma::NeuronId;

use super::{FiberDescriptor, NeuronDescriptor};

/// Loaded subgraph rooted at `root`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubgraphSnapshot {
    /// Root neuron id.
    pub root: NeuronId,
    /// Storage revision for this subgraph.
    pub revision: u64,
    /// Neurons in the subgraph.
    pub neurons: Vec<NeuronDescriptor>,
    /// Fibers in the subgraph.
    pub fibers: Vec<FiberDescriptor>,
}
