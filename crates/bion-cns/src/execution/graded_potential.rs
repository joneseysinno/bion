//! Derived / accumulated signal before threshold.

use bion_soma::Impulse;

/// Sub-threshold accumulation at a ganglion.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GradedPotential {
    /// Accumulated impulse (if any).
    pub sum: Option<Impulse>,
}
