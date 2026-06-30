//! Partial circuit loaded from a subgraph snapshot.

use bion_soma::NeuronId;

use crate::circuit::ganglion::GanglionInstance;
use crate::circuit::synapse::Synapse;

/// Mutable fragment merged into the live [`crate::circuit::circuit::Circuit`].
#[derive(Debug, Clone, Default)]
pub struct CircuitFragment {
    /// Root neuron for this fragment.
    pub root: Option<NeuronId>,
    /// Ganglion instances in this fragment.
    pub ganglia: Vec<GanglionInstance>,
    /// Synapses in this fragment.
    pub synapses: Vec<Synapse>,
}
