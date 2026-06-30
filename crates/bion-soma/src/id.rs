//! Identity primitives — opaque handles and pure deterministic derivation.

use core::fmt;
use core::num::NonZeroU64;

/// An opaque, unique identifier for a Neuron.
///
/// See [`IdSeed`] for minting. Use `Option<NeuronId>` for absence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NeuronId(NonZeroU64);

impl NeuronId {
    pub(crate) const fn from_nonzero(raw: NonZeroU64) -> Self {
        Self(raw)
    }

    /// Expose the raw value for serialization / bridge mapping only.
    #[cfg(feature = "bridge")]
    pub const fn as_raw(self) -> NonZeroU64 {
        self.0
    }

    /// Reconstruct from a raw non-zero value (bridge / deserialization only).
    #[cfg(feature = "bridge")]
    pub const fn from_raw(raw: NonZeroU64) -> Self {
        Self::from_nonzero(raw)
    }
}

impl fmt::Display for NeuronId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NeuronId({})", self.0)
    }
}

/// An opaque, unique identifier for a Fiber.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FiberId(NonZeroU64);

impl FiberId {
    pub(crate) const fn from_nonzero(raw: NonZeroU64) -> Self {
        Self(raw)
    }

    /// Expose the raw value for serialization / bridge mapping only.
    #[cfg(feature = "bridge")]
    pub const fn as_raw(self) -> NonZeroU64 {
        self.0
    }

    /// Reconstruct from a raw non-zero value (bridge / deserialization only).
    #[cfg(feature = "bridge")]
    pub const fn from_raw(raw: NonZeroU64) -> Self {
        Self::from_nonzero(raw)
    }
}

impl fmt::Display for FiberId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FiberId({})", self.0)
    }
}

/// An opaque, unique identifier for a Ganglion.
///
/// Distinct from [`NeuronId`] and [`FiberId`] — accidental substitution is a
/// compile-time error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GanglionId(NonZeroU64);

impl GanglionId {
    pub(crate) const fn from_nonzero(raw: NonZeroU64) -> Self {
        Self(raw)
    }

    /// Expose the raw value for serialization / bridge mapping only.
    #[cfg(feature = "bridge")]
    pub const fn as_raw(self) -> NonZeroU64 {
        self.0
    }
}

impl fmt::Display for GanglionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GanglionId({})", self.0)
    }
}

/// An opaque, unique identifier for a Synapse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SynapseId(NonZeroU64);

impl SynapseId {
    pub(crate) const fn from_nonzero(raw: NonZeroU64) -> Self {
        Self(raw)
    }

    /// Expose the raw value for serialization / bridge mapping only.
    #[cfg(feature = "bridge")]
    pub const fn as_raw(self) -> NonZeroU64 {
        self.0
    }
}

impl fmt::Display for SynapseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SynapseId({})", self.0)
    }
}

/// A pure, deterministic identifier seed.
///
/// Minting consumes the seed and returns `(id, successor)`. Same seed in ⇒
/// same `(id, next)` out. No `&mut`, no entropy, no clock.
///
/// Four independent counters cover Neuron, Fiber, Ganglion, and Synapse id
/// spaces — `Neuron(1)` and `Fiber(1)` may coexist as distinct types.
///
/// # Effects live above Soma
/// Ambient allocation, UUID/entropy, and db-assigned ids belong in
/// `bion-store` as impure adapters over [`IdSource`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdSeed {
    neuron: NonZeroU64,
    fiber: NonZeroU64,
    ganglion: NonZeroU64,
    synapse: NonZeroU64,
}

impl IdSeed {
    /// The origin seed. All counters start at 1 (`NonZeroU64::MIN`).
    pub const fn first() -> Self {
        Self {
            neuron: NonZeroU64::MIN,
            fiber: NonZeroU64::MIN,
            ganglion: NonZeroU64::MIN,
            synapse: NonZeroU64::MIN,
        }
    }

    /// Resume from explicit raw counters. Returns `None` if any value is zero.
    pub const fn from_raw(
        neuron: u64,
        fiber: u64,
        ganglion: u64,
        synapse: u64,
    ) -> Option<Self> {
        match (
            NonZeroU64::new(neuron),
            NonZeroU64::new(fiber),
            NonZeroU64::new(ganglion),
            NonZeroU64::new(synapse),
        ) {
            (Some(n), Some(f), Some(g), Some(s)) => Some(Self {
                neuron: n,
                fiber: f,
                ganglion: g,
                synapse: s,
            }),
            _ => None,
        }
    }

    /// Mint a [`NeuronId`] and the successor seed. Total — never panics.
    pub const fn mint_neuron(self) -> Option<(NeuronId, IdSeed)> {
        let id = NeuronId::from_nonzero(self.neuron);
        match self.neuron.checked_add(1) {
            Some(next) => Some((
                id,
                Self {
                    neuron: next,
                    fiber: self.fiber,
                    ganglion: self.ganglion,
                    synapse: self.synapse,
                },
            )),
            None => None,
        }
    }

    /// Mint a [`FiberId`] and the successor seed. Total — never panics.
    pub const fn mint_fiber(self) -> Option<(FiberId, IdSeed)> {
        let id = FiberId::from_nonzero(self.fiber);
        match self.fiber.checked_add(1) {
            Some(next) => Some((
                id,
                Self {
                    neuron: self.neuron,
                    fiber: next,
                    ganglion: self.ganglion,
                    synapse: self.synapse,
                },
            )),
            None => None,
        }
    }

    /// Mint a [`GanglionId`] and the successor seed. Total — never panics.
    pub const fn mint_ganglion(self) -> Option<(GanglionId, IdSeed)> {
        let id = GanglionId::from_nonzero(self.ganglion);
        match self.ganglion.checked_add(1) {
            Some(next) => Some((
                id,
                Self {
                    neuron: self.neuron,
                    fiber: self.fiber,
                    ganglion: next,
                    synapse: self.synapse,
                },
            )),
            None => None,
        }
    }

    /// Mint a [`SynapseId`] and the successor seed. Total — never panics.
    pub const fn mint_synapse(self) -> Option<(SynapseId, IdSeed)> {
        let id = SynapseId::from_nonzero(self.synapse);
        match self.synapse.checked_add(1) {
            Some(next) => Some((
                id,
                Self {
                    neuron: self.neuron,
                    fiber: self.fiber,
                    ganglion: self.ganglion,
                    synapse: next,
                },
            )),
            None => None,
        }
    }
}

/// A pure source of fresh identifiers.
///
/// Implementors are *values*: minting consumes `self` and returns the id plus
/// the successor source. Effectful allocation lives in `bion-store`.
pub trait IdSource: Sized {
    /// Mint a neuron id and the successor source, or `None` if exhausted.
    fn mint_neuron(self) -> Option<(NeuronId, Self)>;
    /// Mint a fiber id and the successor source, or `None` if exhausted.
    fn mint_fiber(self) -> Option<(FiberId, Self)>;
    /// Mint a ganglion id and the successor source, or `None` if exhausted.
    fn mint_ganglion(self) -> Option<(GanglionId, Self)>;
    /// Mint a synapse id and the successor source, or `None` if exhausted.
    fn mint_synapse(self) -> Option<(SynapseId, Self)>;
}

impl IdSource for IdSeed {
    fn mint_neuron(self) -> Option<(NeuronId, Self)> {
        IdSeed::mint_neuron(self)
    }

    fn mint_fiber(self) -> Option<(FiberId, Self)> {
        IdSeed::mint_fiber(self)
    }

    fn mint_ganglion(self) -> Option<(GanglionId, Self)> {
        IdSeed::mint_ganglion(self)
    }

    fn mint_synapse(self) -> Option<(SynapseId, Self)> {
        IdSeed::mint_synapse(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mint_neuron_is_deterministic() {
        let seed = IdSeed::first();
        let (a, next_a) = seed.mint_neuron().unwrap();
        let (b, next_b) = seed.mint_neuron().unwrap();
        assert_eq!(a, b);
        assert_eq!(next_a, next_b);
    }

    #[test]
    fn chained_mints_produce_distinct_neuron_ids() {
        let (a, seed) = IdSeed::first().mint_neuron().unwrap();
        let (b, _) = seed.mint_neuron().unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn chained_mints_produce_distinct_ganglion_ids() {
        let (a, seed) = IdSeed::first().mint_ganglion().unwrap();
        let (b, _) = seed.mint_ganglion().unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn all_id_spaces_are_independent() {
        let seed = IdSeed::first();
        let (n, seed) = seed.mint_neuron().unwrap();
        let (f, seed) = seed.mint_fiber().unwrap();
        let (g, seed) = seed.mint_ganglion().unwrap();
        let (s, _) = seed.mint_synapse().unwrap();
        let _ = (n, f, g, s);
    }

    #[test]
    fn mint_neuron_at_max_returns_none_without_panic() {
        let seed = IdSeed {
            neuron: NonZeroU64::MAX,
            fiber: NonZeroU64::MIN,
            ganglion: NonZeroU64::MIN,
            synapse: NonZeroU64::MIN,
        };
        assert!(seed.mint_neuron().is_none());
    }

    #[test]
    fn mint_neuron_exhausts_after_penultimate() {
        let penultimate = NonZeroU64::new(u64::MAX - 1).unwrap();
        let seed = IdSeed {
            neuron: penultimate,
            fiber: NonZeroU64::MIN,
            ganglion: NonZeroU64::MIN,
            synapse: NonZeroU64::MIN,
        };
        let (_, exhausted) = seed.mint_neuron().unwrap();
        assert!(exhausted.mint_neuron().is_none());
    }

    #[cfg(feature = "bridge")]
    #[test]
    fn as_raw_is_monotonic_for_neurons() {
        let (a, seed) = IdSeed::first().mint_neuron().unwrap();
        let (b, _) = seed.mint_neuron().unwrap();
        assert!(a.as_raw().get() < b.as_raw().get());
    }
}
