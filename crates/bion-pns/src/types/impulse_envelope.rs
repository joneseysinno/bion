//! A typed signal arriving from any transport.

use bion_soma::{Impulse, NeuronId};

/// Same shape whether it came from WebSocket, CLI, or HTTP.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImpulseEnvelope {
    /// Neuron that should receive the impulse.
    pub target: NeuronId,
    /// Typed payload.
    pub impulse: Impulse,
    /// Client revision for optimistic UI reconciliation.
    pub client_revision: Option<u64>,
}
