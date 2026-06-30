//! Signal propagation rules between ganglia.

use bion_soma::Impulse;

/// Describes how an impulse travels across a synapse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transmission {
    /// Impulse in flight.
    pub impulse: Impulse,
}
