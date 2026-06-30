//! Live graph of ganglion instances.

use bion_soma::{Impulse, NeuronId};

use super::ganglion::GanglionInstance;
use super::synapse::Synapse;
use crate::types::CnsError;

/// The executable circuit — a snapshot of ganglia and synapses.
#[derive(Debug, Default, Clone)]
pub struct Circuit {
    /// Root neuron id for this circuit.
    pub root: Option<NeuronId>,
    /// Live ganglion instances.
    pub ganglia: Vec<GanglionInstance>,
    /// Synapse wiring.
    pub synapses: Vec<Synapse>,
}

impl Circuit {
    /// Apply an impulse to the target ganglion (stub propagation).
    pub fn apply_impulse(&mut self, target: NeuronId, impulse: Impulse) -> Result<Impulse, CnsError> {
        let _ = target;
        if let Some(g) = self.ganglia.first_mut() {
            g.held = Some(impulse.clone());
            return Ok(impulse);
        }
        Err(CnsError::Execution("no ganglia in circuit".into()))
    }
}
