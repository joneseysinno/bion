//! Identity primitives — opaque neuron handles and generators.

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

/// Identity generation service.
///
/// `IdGen` is a trait so that different contexts can use different strategies:
/// - Production: UUID-based (globally unique, safe across distributed nodes)
/// - Testing: Sequential (deterministic, debuggable, reproducible)
/// - Simulation: Seeded random (reproducible chaos)
///
/// Implementors must guarantee that no two calls to `next_id` on the same
/// instance return the same [`NeuronId`] within the instance's lifetime.
pub trait IdGen: Send + Sync {
    /// Mint the next unique [`NeuronId`].
    fn next_id(&mut self) -> NeuronId;
}

/// A deterministic, sequential [`IdGen`] for use in tests and simulations.
///
/// IDs start at 1 because [`NeuronId`] wraps [`NonZeroU64`] — zero is
/// unrepresentable. Use `Option<NeuronId>` for absence at higher layers.
///
/// # Warning
/// Do not use in production. Sequential IDs are predictable and
/// will collide if two instances are created independently.
pub struct SequentialIdGen {
    counter: NonZeroU64,
}

impl SequentialIdGen {
    /// Creates a generator whose first ID is 1.
    pub fn new() -> Self {
        Self {
            counter: NonZeroU64::MIN,
        }
    }
}

impl Default for SequentialIdGen {
    fn default() -> Self {
        Self::new()
    }
}

impl IdGen for SequentialIdGen {
    fn next_id(&mut self) -> NeuronId {
        let id = NeuronId::from_nonzero(self.counter);
        self.counter = self
            .counter
            .checked_add(1)
            .expect("SequentialIdGen exhausted");
        id
    }
}

/// A UUID-based [`IdGen`] for production use.
///
/// Each call generates a new UUIDv4, truncates to u64 via the high 64 bits.
/// If the high bits are zero (astronomically rare), retries until non-zero.
/// Collision probability is astronomically low for any realistic graph size.
///
/// # Design note
/// We truncate UUID to u64 rather than storing the full 128-bit UUID because
/// [`NeuronId`] is used as a map key throughout the hot execution path and
/// u64 hashing is significantly faster than u128. The bridge layer stores
/// the full UUID mapping if needed for external correlation.
pub struct UuidIdGen;

impl IdGen for UuidIdGen {
    fn next_id(&mut self) -> NeuronId {
        loop {
            let uuid = uuid::Uuid::new_v4();
            let (high, _) = uuid.as_u64_pair();
            if let Some(nz) = NonZeroU64::new(high) {
                return NeuronId::from_nonzero(nz);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_produces_distinct_ids() {
        let mut id_gen = SequentialIdGen::new();
        let a = id_gen.next_id();
        let b = id_gen.next_id();
        assert_ne!(a, b);
    }

    #[test]
    fn sequential_is_monotonic_with_bridge() {
        #[cfg(feature = "bridge")]
        {
            let mut id_gen = SequentialIdGen::new();
            let a = id_gen.next_id().as_raw();
            let b = id_gen.next_id().as_raw();
            assert!(a < b);
        }
    }

    #[test]
    fn uuid_produces_distinct_ids() {
        let mut id_gen = UuidIdGen;
        let ids: Vec<NeuronId> = (0..32).map(|_| id_gen.next_id()).collect();
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
            assert_ne!(seq_gen.next_id().as_raw(), 0);
        }
        let mut uuid_gen = UuidIdGen;
        for _ in 0..10 {
            assert_ne!(uuid_gen.next_id().as_raw(), 0);
        }
    }
}
