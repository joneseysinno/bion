//! The only surface CNS uses to read graph definitions.

use async_trait::async_trait;
use bion_soma::NeuronId;

use crate::types::{FiberDescriptor, NeuronDescriptor, PnsError, SubgraphSnapshot};

/// Implemented over the real db client and over an in-memory mock for tests.
#[async_trait]
pub trait PnsReader: Send + Sync {
    /// Load a single neuron descriptor.
    async fn fetch_neuron(&self, id: NeuronId) -> Result<NeuronDescriptor, PnsError>;
    /// Load a subgraph rooted at `root`.
    async fn fetch_subgraph(&self, root: NeuronId) -> Result<SubgraphSnapshot, PnsError>;
    /// List fibers attached to `neuron`.
    async fn fetch_fibers(&self, neuron: NeuronId) -> Result<Vec<FiberDescriptor>, PnsError>;
}
