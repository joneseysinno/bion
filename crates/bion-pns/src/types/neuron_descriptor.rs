//! Neuron metadata loaded from storage.

use bion_soma::{NeuronId, NeuronType};

/// Descriptor for a single neuron node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeuronDescriptor {
    /// Neuron identity.
    pub id: NeuronId,
    /// Structural role.
    pub neuron_type: NeuronType,
    /// Storage revision.
    pub revision: u64,
}
