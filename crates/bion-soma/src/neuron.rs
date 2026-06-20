//! Neuron functional roles and structural capability flags.

/// The functional role of a Neuron within the biological graph.
///
/// `NeuronType` is not a runtime behavior tag — it does not change how
/// signals are computed. It is a *structural role* that governs:
/// - Where in the graph a Neuron may legally appear
/// - Which Fiber polarities are valid for this role
/// - How the Genesis editor renders this node on the canvas
///
/// Think of it as the cell type in biology: a neuron's type determines
/// its morphology and position in the nervous system, not the chemistry
/// of any individual signal it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NeuronType {
    /// Receives input from outside the system (world → graph boundary).
    Sensory,

    /// Produces effects outside the system (graph → world boundary).
    Motor,

    /// Internal processing node. No direct connection to the world surface.
    Interneuron,

    /// Stateful node. Holds a value across signal wavefronts.
    Memory,

    /// Clock node. Fires on a schedule independent of input signals.
    Pacemaker,
}

impl NeuronType {
    /// Returns true if this NeuronType may appear at the graph boundary
    /// (connected to Membrane Receptors or Effectors).
    pub const fn is_boundary(self) -> bool {
        matches!(self, NeuronType::Sensory | NeuronType::Motor)
    }

    /// Returns true if this NeuronType can initiate a wavefront
    /// without receiving an external Impulse.
    pub const fn is_autonomous(self) -> bool {
        matches!(self, NeuronType::Pacemaker)
    }

    /// Returns true if this NeuronType carries state across wavefronts.
    pub const fn is_stateful(self) -> bool {
        matches!(self, NeuronType::Memory)
    }

    /// Structural capability flags for this role.
    ///
    /// Prefer this when Cortex or Genesis need multiple flags at once —
    /// avoids boolean proliferation at call sites.
    pub const fn capabilities(self) -> NeuronCapabilities {
        NeuronCapabilities {
            boundary: self.is_boundary(),
            autonomous: self.is_autonomous(),
            stateful: self.is_stateful(),
        }
    }
}

/// Structural capability flags derived from a [`NeuronType`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NeuronCapabilities {
    /// May appear at the graph boundary (Sensory / Motor).
    pub boundary: bool,
    /// Can initiate a wavefront without external input (Pacemaker).
    pub autonomous: bool,
    /// Carries state across wavefronts (Memory).
    pub stateful: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundary_only_sensory_and_motor() {
        assert!(NeuronType::Sensory.is_boundary());
        assert!(NeuronType::Motor.is_boundary());
        assert!(!NeuronType::Interneuron.is_boundary());
        assert!(!NeuronType::Memory.is_boundary());
        assert!(!NeuronType::Pacemaker.is_boundary());
    }

    #[test]
    fn autonomous_only_pacemaker() {
        assert!(NeuronType::Pacemaker.is_autonomous());
        assert!(!NeuronType::Sensory.is_autonomous());
        assert!(!NeuronType::Motor.is_autonomous());
        assert!(!NeuronType::Interneuron.is_autonomous());
        assert!(!NeuronType::Memory.is_autonomous());
    }

    #[test]
    fn stateful_only_memory() {
        assert!(NeuronType::Memory.is_stateful());
        assert!(!NeuronType::Sensory.is_stateful());
        assert!(!NeuronType::Motor.is_stateful());
        assert!(!NeuronType::Interneuron.is_stateful());
        assert!(!NeuronType::Pacemaker.is_stateful());
    }

    #[test]
    fn capabilities_match_individual_predicates() {
        for ty in [
            NeuronType::Sensory,
            NeuronType::Motor,
            NeuronType::Interneuron,
            NeuronType::Memory,
            NeuronType::Pacemaker,
        ] {
            let caps = ty.capabilities();
            assert_eq!(caps.boundary, ty.is_boundary());
            assert_eq!(caps.autonomous, ty.is_autonomous());
            assert_eq!(caps.stateful, ty.is_stateful());
        }
    }
}
