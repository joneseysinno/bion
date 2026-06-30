//! Live signal in flight.

use bion_soma::Impulse;

/// One firing event propagating through the circuit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionPotential {
    /// Payload carried by this potential.
    pub impulse: Impulse,
}
