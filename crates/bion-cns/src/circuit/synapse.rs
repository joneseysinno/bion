//! Synapse identity and wiring mode.

pub use bion_soma::SynapseId;

/// How a synapse propagates signals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynapticMode {
    /// Forward on fire.
    Excitatory,
    /// Reserved — inhibition deferred.
    Inhibitory,
}

/// Directed connection between ganglia.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Synapse {
    /// Synapse identity.
    pub id: SynapseId,
    /// Propagation mode.
    pub mode: SynapticMode,
}
