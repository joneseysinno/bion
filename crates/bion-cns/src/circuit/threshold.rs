//! Firing threshold policy.

/// When a ganglion emits an action potential.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Threshold {
    /// Minimum accumulated signal count before firing.
    pub min_inputs: u32,
}
