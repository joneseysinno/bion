//! Fiber port in the execution graph.

pub use bion_soma::FiberId;

/// Runtime fiber binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fiber {
    /// Fiber identity.
    pub id: FiberId,
    /// Signal schema.
    pub signal_type: bion_soma::SignalType,
}
