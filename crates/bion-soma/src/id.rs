//! Identity primitives — opaque neuron and fiber handles and pure derivation.

use core::fmt;
use core::num::NonZeroU64;

/// An opaque, unique identifier for a Neuron.
///
/// `NeuronId` carries no semantic meaning beyond identity. It does not know
/// what kind of Neuron it identifies, what connections it has, or whether
/// the Neuron currently exists. It is a name tag, not a reference.
///
/// # Design note
/// The inner [`NonZeroU64`] is private. Nothing outside this module should
/// construct a `NeuronId` by raw value — only [`IdSeed`] / [`IdSource`] mint them.
/// This prevents accidentally valid-looking IDs from being fabricated and
/// makes zero an unrepresentable ID (use `Option<NeuronId>` for absence).
///
/// # ID reuse and generation safety
/// `NeuronId` does not include a generation counter. Once a `NeuronId` is
/// retired (its Neuron deleted), the same numeric value may in principle be
/// issued to a new Neuron. In practice:
///
/// - [`IdSeed`] never reuses IDs within a deterministic minting session
///   (monotonic counters). Across sessions (e.g. loading from a database),
///   IDs are assigned by the stored genome — the database is the source of
///   truth, not the seed.
///
/// - Entropy-based schemes (UUID, etc.) live in `bion-store` as impure adapters.
///
/// If you need stale-reference detection (holding a `NeuronId` to a Neuron
/// that has since been deleted), the store layer is responsible — check
/// existence via `NeuronStore::exists(id)` rather than relying on ID uniqueness
/// alone. Do not add a generation counter to `NeuronId` without a full design
/// review of how the store layer and bridge layer interact with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NeuronId(NonZeroU64);

impl NeuronId {
    /// Private constructor — only callable within this crate.
    pub(crate) const fn from_nonzero(raw: NonZeroU64) -> Self {
        Self(raw)
    }

    /// Expose the raw value for serialization / bridge mapping only.
    ///
    /// # Bridge only
    /// Enabled with the `bridge` feature. Do not use this to construct IDs —
    /// use [`IdSeed`] or [`IdSource`]. Application code must not depend on raw ID values.
    #[cfg(feature = "bridge")]
    pub const fn as_raw(self) -> NonZeroU64 {
        self.0
    }
}

impl fmt::Display for NeuronId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NeuronId({})", self.0)
    }
}

/// An opaque, unique identifier for a Fiber.
///
/// A Fiber is a typed, polarized connection point on a Ganglion.
/// `FiberId` identifies the port, not the signal flowing through it,
/// and not the Synapse it may eventually form.
///
/// Like [`NeuronId`], `FiberId` carries no semantic meaning beyond identity.
/// It does not know which Ganglion owns it, what [`Polarity`](crate::Polarity)
/// it has, or what [`SignalType`](crate::SignalType) it carries — those are
/// Connectome-layer concerns.
///
/// # Design note
/// `FiberId` is intentionally separate from [`NeuronId`]. A Fiber is not
/// a Neuron. Sharing an ID type would allow accidental substitution
/// (passing a `NeuronId` where a `FiberId` is expected) — a class of
/// bug the type system should prevent entirely.
///
/// # ID reuse and generation safety
/// `FiberId` does not include a generation counter. See [`NeuronId`] for the
/// full reuse policy. Impure allocation (entropy, ambient counters, db-assigned
/// ids) belongs in `bion-store`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FiberId(NonZeroU64);

impl FiberId {
    /// Private constructor — only callable within this crate.
    pub(crate) const fn from_nonzero(raw: NonZeroU64) -> Self {
        Self(raw)
    }

    /// Expose the raw value for serialization / bridge mapping only.
    ///
    /// # Bridge only
    /// Enabled with the `bridge` feature. Do not use this to construct IDs —
    /// use [`IdSeed`] or [`IdSource`]. Application code must not depend on raw ID values.
    #[cfg(feature = "bridge")]
    pub const fn as_raw(self) -> NonZeroU64 {
        self.0
    }
}

impl fmt::Display for FiberId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FiberId({})", self.0)
    }
}

/// A pure, deterministic identifier seed.
///
/// `IdSeed` is a *value*, not a generator with hidden state. Minting consumes
/// the seed and returns the id together with the successor seed:
///
/// ```text
/// (id_0, seed_1) = seed_0.mint_neuron().unwrap();
/// (id_1, seed_2) = seed_1.mint_neuron().unwrap();
/// ```
///
/// Given the same seed, `mint_*` always returns the same `(id, next)` pair —
/// referential transparency holds. There is no `&mut`, no interior mutability,
/// no entropy, no clock.
///
/// Neuron and Fiber ids draw from independent counters, so `Neuron(1)` and
/// `Fiber(1)` may coexist.
///
/// # Effects live above Soma
/// Ambient/`&mut` allocation, UUID/entropy schemes, and db-assigned ids are
/// all *effects*. They belong in `bion-store` as impure adapters that wrap a
/// pure [`IdSource`]. Soma only defines deterministic derivation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdSeed {
    neuron: NonZeroU64,
    fiber: NonZeroU64,
}

impl IdSeed {
    /// The origin seed. Both counters start at 1 (`NonZeroU64::MIN`).
    pub const fn first() -> Self {
        Self {
            neuron: NonZeroU64::MIN,
            fiber: NonZeroU64::MIN,
        }
    }

    /// Resume from explicit raw counters (e.g. rehydrating from a stored
    /// genome). Returns `None` if either raw value is zero.
    pub const fn from_raw(neuron: u64, fiber: u64) -> Option<Self> {
        match (NonZeroU64::new(neuron), NonZeroU64::new(fiber)) {
            (Some(n), Some(f)) => Some(Self { neuron: n, fiber: f }),
            _ => None,
        }
    }

    /// Mint a [`NeuronId`] and the successor seed.
    ///
    /// Returns `None` when the neuron counter would overflow `u64`.
    /// Total — never panics.
    pub const fn mint_neuron(self) -> Option<(NeuronId, IdSeed)> {
        let id = NeuronId::from_nonzero(self.neuron);
        match self.neuron.checked_add(1) {
            Some(next) => Some((
                id,
                IdSeed {
                    neuron: next,
                    fiber: self.fiber,
                },
            )),
            None => None,
        }
    }

    /// Mint a [`FiberId`] and the successor seed.
    ///
    /// Returns `None` when the fiber counter would overflow `u64`.
    /// Total — never panics.
    pub const fn mint_fiber(self) -> Option<(FiberId, IdSeed)> {
        let id = FiberId::from_nonzero(self.fiber);
        match self.fiber.checked_add(1) {
            Some(next) => Some((
                id,
                IdSeed {
                    neuron: self.neuron,
                    fiber: next,
                },
            )),
            None => None,
        }
    }
}

/// A pure source of fresh identifiers.
///
/// Implementors are *values*: minting consumes `self` and returns the id plus
/// the successor source. No `&mut`, no interior mutability, no entropy, no clock.
///
/// All effectful allocation — ambient counters, UUID/entropy, db-assigned ids —
/// lives **above Soma** (in `bion-store`) as an impure adapter that drives a
/// pure `IdSource` internally.
pub trait IdSource: Sized {
    /// Mint a neuron id and the successor source, or `None` if exhausted.
    fn mint_neuron(self) -> Option<(NeuronId, Self)>;
    /// Mint a fiber id and the successor source, or `None` if exhausted.
    fn mint_fiber(self) -> Option<(FiberId, Self)>;
}

impl IdSource for IdSeed {
    fn mint_neuron(self) -> Option<(NeuronId, Self)> {
        IdSeed::mint_neuron(self)
    }

    fn mint_fiber(self) -> Option<(FiberId, Self)> {
        IdSeed::mint_fiber(self)
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
    fn mint_fiber_is_deterministic() {
        let seed = IdSeed::first();
        let (a, next_a) = seed.mint_fiber().unwrap();
        let (b, next_b) = seed.mint_fiber().unwrap();
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
    fn chained_mints_produce_distinct_fiber_ids() {
        let (a, seed) = IdSeed::first().mint_fiber().unwrap();
        let (b, _) = seed.mint_fiber().unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn neuron_and_fiber_counters_are_independent() {
        let seed = IdSeed::first();
        let (n1, seed) = seed.mint_neuron().unwrap();
        let (f1, seed) = seed.mint_fiber().unwrap();
        let (n2, _) = seed.mint_neuron().unwrap();
        let _ = (n1, f1, n2);
    }

    #[test]
    fn mint_neuron_at_max_returns_none_without_panic() {
        let seed = IdSeed {
            neuron: NonZeroU64::MAX,
            fiber: NonZeroU64::MIN,
        };
        assert!(seed.mint_neuron().is_none());
    }

    #[test]
    fn mint_neuron_exhausts_after_penultimate() {
        let penultimate = NonZeroU64::new(u64::MAX - 1).unwrap();
        let seed = IdSeed {
            neuron: penultimate,
            fiber: NonZeroU64::MIN,
        };
        let (_, exhausted) = seed.mint_neuron().unwrap();
        assert!(exhausted.mint_neuron().is_none());
    }

    #[test]
    fn mint_fiber_at_max_returns_none_without_panic() {
        let seed = IdSeed {
            neuron: NonZeroU64::MIN,
            fiber: NonZeroU64::MAX,
        };
        assert!(seed.mint_fiber().is_none());
    }

    #[test]
    fn mint_fiber_exhausts_after_penultimate() {
        let penultimate = NonZeroU64::new(u64::MAX - 1).unwrap();
        let seed = IdSeed {
            neuron: NonZeroU64::MIN,
            fiber: penultimate,
        };
        let (_, exhausted) = seed.mint_fiber().unwrap();
        assert!(exhausted.mint_fiber().is_none());
    }

    #[test]
    fn id_source_trait_matches_id_seed() {
        let seed = IdSeed::first();
        let via_trait = IdSource::mint_neuron(seed).unwrap();
        let via_method = seed.mint_neuron().unwrap();
        assert_eq!(via_trait.0, via_method.0);
        assert_eq!(via_trait.1, via_method.1);
    }

    #[cfg(feature = "bridge")]
    #[test]
    fn as_raw_is_monotonic_for_neurons() {
        let (a, seed) = IdSeed::first().mint_neuron().unwrap();
        let (b, _) = seed.mint_neuron().unwrap();
        assert!(a.as_raw().get() < b.as_raw().get());
    }

    #[cfg(feature = "bridge")]
    #[test]
    fn as_raw_never_zero() {
        let (n, seed) = IdSeed::first().mint_neuron().unwrap();
        let (f, _) = seed.mint_fiber().unwrap();
        assert_ne!(n.as_raw().get(), 0);
        assert_ne!(f.as_raw().get(), 0);
    }
}
