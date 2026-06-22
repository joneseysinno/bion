//! Fiber directional orientation and validated synapse wiring.

use core::fmt;

/// The directional orientation of a Fiber connection point.
///
/// Every Fiber has a Polarity. A Synapse must connect an `Efferent` Fiber
/// on the emitting side to an `Afferent` Fiber on the receiving side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Polarity {
    /// Receives — carries signals into the owning Ganglion.
    Afferent,

    /// Emits — carries signals out of the owning Ganglion.
    Efferent,
}

impl Polarity {
    /// Returns the opposite polarity.
    ///
    /// # Law
    /// `p.opposite().opposite() == p` (involution).
    pub const fn opposite(self) -> Polarity {
        match self {
            Polarity::Afferent => Polarity::Efferent,
            Polarity::Efferent => Polarity::Afferent,
        }
    }

    /// Returns true if this polarity is compatible as a Synapse target
    /// for a Fiber with the given `source` polarity.
    ///
    /// Valid: Efferent → Afferent
    pub const fn can_connect_to(self, target: Polarity) -> bool {
        matches!((self, target), (Polarity::Efferent, Polarity::Afferent))
    }
}

impl fmt::Display for Polarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Polarity::Afferent => write!(f, "Afferent (←)"),
            Polarity::Efferent => write!(f, "Efferent (→)"),
        }
    }
}

/// A validated Efferent → Afferent connection.
///
/// Construct only via [`ValidSynapse::new`] — makes a legal wiring orientation
/// unforgeable without passing the structural check (parse, don't validate).
///
/// # Example: direct construction is rejected
///
/// ```compile_fail
/// use bion_soma::{Polarity, ValidSynapse};
/// let _ = ValidSynapse {
///     source: Polarity::Afferent,
///     sink: Polarity::Efferent,
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValidSynapse {
    source: Polarity,
    sink: Polarity,
}

impl ValidSynapse {
    /// Returns `Some` only when `source.can_connect_to(sink)`.
    pub fn new(source: Polarity, sink: Polarity) -> Option<Self> {
        if source.can_connect_to(sink) {
            Some(Self { source, sink })
        } else {
            None
        }
    }

    /// Returns the emitting (Efferent) side.
    pub const fn source(self) -> Polarity {
        self.source
    }

    /// Returns the receiving (Afferent) side.
    pub const fn sink(self) -> Polarity {
        self.sink
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite_involution() {
        for p in [Polarity::Afferent, Polarity::Efferent] {
            assert_eq!(p.opposite().opposite(), p);
        }
    }

    #[test]
    fn can_connect_to_efferent_to_afferent_only() {
        assert!(Polarity::Efferent.can_connect_to(Polarity::Afferent));
        assert!(!Polarity::Afferent.can_connect_to(Polarity::Afferent));
        assert!(!Polarity::Efferent.can_connect_to(Polarity::Efferent));
        assert!(!Polarity::Afferent.can_connect_to(Polarity::Efferent));
    }

    #[test]
    fn valid_synapse_matches_can_connect_to() {
        assert!(ValidSynapse::new(Polarity::Efferent, Polarity::Afferent).is_some());
        assert!(ValidSynapse::new(Polarity::Afferent, Polarity::Afferent).is_none());
        assert!(ValidSynapse::new(Polarity::Efferent, Polarity::Efferent).is_none());
        assert!(ValidSynapse::new(Polarity::Afferent, Polarity::Efferent).is_none());
    }

    #[test]
    fn valid_synapse_accessors() {
        let synapse = ValidSynapse::new(Polarity::Efferent, Polarity::Afferent).unwrap();
        assert_eq!(synapse.source(), Polarity::Efferent);
        assert_eq!(synapse.sink(), Polarity::Afferent);
    }
}
