//! Merge a hydrated fragment into the live circuit.

use crate::circuit::circuit::Circuit;
use crate::types::CircuitFragment;

/// Replace ganglia/synapses for the fragment root in `circuit`.
pub fn rebuild_fragment(circuit: &mut Circuit, fragment: CircuitFragment) {
    if let Some(root) = fragment.root {
        circuit.root = Some(root);
    }
    circuit.ganglia = fragment.ganglia;
    circuit.synapses = fragment.synapses;
}
