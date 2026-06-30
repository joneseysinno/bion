//! Fiber port metadata loaded from storage.

use bion_soma::{FiberId, NeuronId, Polarity, SignalType};

/// Descriptor for a single fiber port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiberDescriptor {
    /// Fiber identity.
    pub id: FiberId,
    /// Owning neuron.
    pub neuron: NeuronId,
    /// Signal schema carried on this fiber.
    pub signal_type: SignalType,
    /// Input or output polarity.
    pub polarity: Polarity,
}
