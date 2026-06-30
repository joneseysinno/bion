//! A typed state change pushed to any subscribed frontend.

use bion_soma::{Impulse, NeuronId};

/// Broadcast to WebSocket, CLI, or HTTP subscribers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateUpdate {
    /// Neuron whose state changed.
    pub source: NeuronId,
    /// New impulse value.
    pub impulse: Impulse,
    /// Storage revision after the write.
    pub revision: u64,
}
