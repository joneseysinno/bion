//! Identity primitives — opaque neuron and fiber handles and generators.

use std::fmt;
use std::num::NonZeroU64;

/// An opaque, unique identifier for a Neuron.
///
/// `NeuronId` carries no semantic meaning beyond identity. It does not know
/// what kind of Neuron it identifies, what connections it has, or whether
/// the Neuron currently exists. It is a name tag, not a reference.
///
/// # Design note
/// The inner [`NonZeroU64`] is private. Nothing outside this module should
/// construct a `NeuronId` by raw value — only [`IdGen`] impls mint them.
/// This prevents accidentally valid-looking IDs from being fabricated and
/// makes zero an unrepresentable ID (use `Option<NeuronId>` for absence).
///
/// # ID reuse and generation safety
/// `NeuronId` does not include a generation counter. Once a `NeuronId` is
/// retired (its Neuron deleted), the same numeric value may in principle be
/// issued to a new Neuron. In practice:
///
/// - [`SequentialIdGen`] never reuses IDs within a session (monotonic counter).
///   Across sessions (e.g. loading from a database), IDs are assigned by the
///   stored genome — the database is the source of truth, not the generator.
///
/// - [`UuidIdGen`] never reuses IDs in practice (collision probability ~2^-64).
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
    /// External code must go through an [`IdGen`] impl.
    pub(crate) fn from_nonzero(raw: NonZeroU64) -> Self {
        Self(raw)
    }

    /// Expose the raw value for serialization / bridge mapping only.
    ///
    /// # Bridge only
    /// Enabled with the `bridge` feature. Do not use this to construct IDs —
    /// use [`IdGen`]. Application code must not depend on raw ID values.
    #[cfg(feature = "bridge")]
    pub fn as_raw(self) -> u64 {
        self.0.get()
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
/// `FiberId` does not include a generation counter. Once a `FiberId` is
/// retired (its Fiber deleted), the same numeric value may in principle be
/// issued to a new Fiber. In practice:
///
/// - [`SequentialIdGen`] never reuses IDs within a session (monotonic counter).
///   Across sessions, IDs are assigned by the stored genome — the database
///   is the source of truth, not the generator.
///
/// - [`UuidIdGen`] never reuses IDs in practice (collision probability ~2^-64).
///
/// If you need stale-reference detection, the store layer is responsible.
/// Do not add a generation counter to `FiberId` without a full design review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FiberId(NonZeroU64);

impl FiberId {
    /// Private constructor — only callable within this crate.
    pub(crate) fn from_nonzero(raw: NonZeroU64) -> Self {
        Self(raw)
    }

    /// Expose the raw value for serialization / bridge mapping only.
    ///
    /// # Bridge only
    /// Enabled with the `bridge` feature. Do not use this to construct IDs —
    /// use [`IdGen`]. Application code must not depend on raw ID values.
    #[cfg(feature = "bridge")]
    pub fn as_raw(self) -> NonZeroU64 {
        self.0
    }
}

impl fmt::Display for FiberId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FiberId({})", self.0)
    }
}

/// Identity generation service.
///
/// `IdGen` is a trait so that different contexts can use different strategies:
/// - Production: UUID-based (globally unique, safe across distributed nodes)
/// - Testing: Sequential (deterministic, debuggable, reproducible)
/// - Simulation: Seeded random (reproducible chaos)
///
/// Implementors must guarantee that no two calls to `next_neuron_id` or
/// `next_fiber_id` on the same instance return the same ID within the
/// instance's lifetime.
pub trait IdGen: Send + Sync {
    /// Mint the next unique [`NeuronId`].
    fn next_neuron_id(&mut self) -> NeuronId;

    /// Mint the next unique [`FiberId`].
    fn next_fiber_id(&mut self) -> FiberId;
}

fn next_nonzero_from_uuid() -> NonZeroU64 {
    loop {
        let uuid = uuid::Uuid::new_v4();
        let (high, _) = uuid.as_u64_pair();
        if let Some(nz) = NonZeroU64::new(high) {
            return nz;
        }
    }
}

/// A deterministic, sequential [`IdGen`] for use in tests and simulations.
///
/// IDs start at 1 because [`NeuronId`] and [`FiberId`] wrap [`NonZeroU64`] —
/// zero is unrepresentable. Use `Option<NeuronId>` / `Option<FiberId>` for
/// absence at higher layers.
///
/// # Session scope
/// Counters are monotonic within a single generator instance (one session).
/// IDs are never reused within that session. Across sessions, persisted
/// genomes assign IDs — the store is the source of truth.
///
/// # Warning
/// Do not use in production. Sequential IDs are predictable and
/// will collide if two instances are created independently.
pub struct SequentialIdGen {
    neuron_counter: NonZeroU64,
    fiber_counter: NonZeroU64,
}

impl SequentialIdGen {
    /// Creates a generator whose first neuron and fiber IDs are both 1.
    pub fn new() -> Self {
        Self {
            neuron_counter: NonZeroU64::MIN,
            fiber_counter: NonZeroU64::MIN,
        }
    }
}

impl Default for SequentialIdGen {
    fn default() -> Self {
        Self::new()
    }
}

impl IdGen for SequentialIdGen {
    fn next_neuron_id(&mut self) -> NeuronId {
        let id = NeuronId::from_nonzero(self.neuron_counter);
        self.neuron_counter = self
            .neuron_counter
            .checked_add(1)
            .expect("SequentialIdGen neuron counter exhausted");
        id
    }

    fn next_fiber_id(&mut self) -> FiberId {
        let id = FiberId::from_nonzero(self.fiber_counter);
        self.fiber_counter = self
            .fiber_counter
            .checked_add(1)
            .expect("SequentialIdGen fiber counter exhausted");
        id
    }
}

/// A UUID-based [`IdGen`] for production use.
///
/// Each call generates a new UUIDv4, truncates to u64 via the high 64 bits.
/// If the high bits are zero (astronomically rare), retries until non-zero.
///
/// # Collision probability
/// Truncating UUIDv4 to 64 bits yields collision probability ~50% only after
/// ~2^32 IDs (birthday bound). For realistic Bion graphs this is acceptable.
/// The bridge layer may store full 128-bit UUIDs for external correlation.
///
/// # Design note
/// We truncate UUID to u64 rather than storing the full 128-bit UUID because
/// ID types are used as map keys throughout the hot execution path and
/// u64 hashing is significantly faster than u128.
pub struct UuidIdGen;

impl IdGen for UuidIdGen {
    fn next_neuron_id(&mut self) -> NeuronId {
        NeuronId::from_nonzero(next_nonzero_from_uuid())
    }

    fn next_fiber_id(&mut self) -> FiberId {
        FiberId::from_nonzero(next_nonzero_from_uuid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_produces_distinct_neuron_ids() {
        let mut id_gen = SequentialIdGen::new();
        let a = id_gen.next_neuron_id();
        let b = id_gen.next_neuron_id();
        assert_ne!(a, b);
    }

    #[test]
    fn sequential_produces_distinct_fiber_ids() {
        let mut id_gen = SequentialIdGen::new();
        let a = id_gen.next_fiber_id();
        let b = id_gen.next_fiber_id();
        assert_ne!(a, b);
    }

    #[test]
    fn neuron_and_fiber_ids_are_distinct_types() {
        let mut id_gen = SequentialIdGen::new();
        let _neuron: NeuronId = id_gen.next_neuron_id();
        let _fiber: FiberId = id_gen.next_fiber_id();
    }

    #[test]
    fn sequential_is_monotonic_with_bridge() {
        #[cfg(feature = "bridge")]
        {
            let mut id_gen = SequentialIdGen::new();
            let a = id_gen.next_neuron_id().as_raw();
            let b = id_gen.next_neuron_id().as_raw();
            assert!(a < b);
        }
    }

    #[test]
    fn uuid_produces_distinct_neuron_ids() {
        let mut id_gen = UuidIdGen;
        let ids: Vec<NeuronId> = (0..32).map(|_| id_gen.next_neuron_id()).collect();
        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                assert_ne!(ids[i], ids[j]);
            }
        }
    }

    #[cfg(feature = "bridge")]
    #[test]
    fn as_raw_never_zero() {
        let mut seq_gen = SequentialIdGen::new();
        for _ in 0..10 {
            assert_ne!(seq_gen.next_neuron_id().as_raw(), 0);
            assert_ne!(seq_gen.next_fiber_id().as_raw().get(), 0);
        }
        let mut uuid_gen = UuidIdGen;
        for _ in 0..10 {
            assert_ne!(uuid_gen.next_neuron_id().as_raw(), 0);
            assert_ne!(uuid_gen.next_fiber_id().as_raw().get(), 0);
        }
    }
}
