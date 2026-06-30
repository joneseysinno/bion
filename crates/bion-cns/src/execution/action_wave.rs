//! Parallel wavefront of action potentials.

use super::action_potential::ActionPotential;

/// A batch of simultaneous firings.
#[derive(Debug, Default, Clone)]
pub struct ActionWave {
    /// Potentials in this wavefront.
    pub potentials: Vec<ActionPotential>,
}
