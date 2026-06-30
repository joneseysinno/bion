//! Neuron functional roles and structural capability flags.

use crate::polarity::Polarity;

/// Fiber count contract for a given role and polarity.
///
/// Uses named fields on [`Arity::Bounded`] so min/max cannot be transposed
/// silently — no naked `(u32, u32)` tuples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Arity {
    /// This polarity is ontologically forbidden for this role (count must be 0).
    Forbidden,
    /// Exactly this many fibers required.
    Exactly(u32),
    /// Zero or more — at least this many.
    AtLeast(u32),
    /// Between `min` and `max` inclusive.
    Bounded {
        /// Minimum fiber count (inclusive).
        min: u32,
        /// Maximum fiber count (inclusive).
        max: u32,
    },
}

/// The functional role of a Neuron within the biological graph.
///
/// `NeuronType` is not a runtime behavior tag — it does not change how
/// signals are computed. It is a *structural role* that governs:
/// - Where in the graph a Neuron may legally appear
/// - Which Fiber polarities are valid for this role
/// - How the Genesis editor renders this node on the canvas
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
    pub const fn capabilities(self) -> NeuronCapabilities {
        NeuronCapabilities {
            boundary: self.is_boundary(),
            autonomous: self.is_autonomous(),
            stateful: self.is_stateful(),
        }
    }

    /// Structural fiber arity contract for this role and polarity.
    ///
    /// Encodes only *ontological* minimums and impossibilities. Design
    /// preferences (e.g. "should have exactly one efferent for layout") stay
    /// in Cortex `Immune`.
    pub const fn fiber_arity(self, polarity: Polarity) -> Arity {
        match (self, polarity) {
            // Sensory: world→graph boundary. Cannot receive afferent input from
            // the graph interior on its external surface — ontological impossibility.
            (NeuronType::Sensory, Polarity::Afferent) => Arity::Forbidden,
            // Must emit at least one efferent to carry world input inward.
            (NeuronType::Sensory, Polarity::Efferent) => Arity::AtLeast(1),

            // Motor: graph→world boundary. Cannot emit efferent to the world
            // surface — effects are received, not sent, at the boundary.
            (NeuronType::Motor, Polarity::Efferent) => Arity::Forbidden,
            // Must receive at least one afferent to carry output outward.
            (NeuronType::Motor, Polarity::Afferent) => Arity::AtLeast(1),

            // Pacemaker: autonomous — cannot depend on external afferent input
            // to fire. Policy about efferent count stays in Cortex.
            (NeuronType::Pacemaker, Polarity::Afferent) => Arity::Forbidden,
            (NeuronType::Pacemaker, Polarity::Efferent) => Arity::AtLeast(0),

            // Interneuron / Memory: internal roles — no ontological polarity ban.
            (NeuronType::Interneuron | NeuronType::Memory, _) => Arity::AtLeast(0),
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
    }

    #[test]
    fn sensory_afferent_forbidden_efferent_required() {
        assert_eq!(
            NeuronType::Sensory.fiber_arity(Polarity::Afferent),
            Arity::Forbidden
        );
        assert_eq!(
            NeuronType::Sensory.fiber_arity(Polarity::Efferent),
            Arity::AtLeast(1)
        );
    }

    #[test]
    fn motor_efferent_forbidden_afferent_required() {
        assert_eq!(
            NeuronType::Motor.fiber_arity(Polarity::Efferent),
            Arity::Forbidden
        );
        assert_eq!(
            NeuronType::Motor.fiber_arity(Polarity::Afferent),
            Arity::AtLeast(1)
        );
    }

    #[test]
    fn pacemaker_afferent_forbidden() {
        assert_eq!(
            NeuronType::Pacemaker.fiber_arity(Polarity::Afferent),
            Arity::Forbidden
        );
    }

    #[test]
    fn interneuron_allows_both_polarities() {
        assert_eq!(
            NeuronType::Interneuron.fiber_arity(Polarity::Afferent),
            Arity::AtLeast(0)
        );
        assert_eq!(
            NeuronType::Interneuron.fiber_arity(Polarity::Efferent),
            Arity::AtLeast(0)
        );
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
