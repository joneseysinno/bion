//! Minimal stand-in until the real infinite-db workspace crate is linked.

use core::num::NonZeroU64;

/// Synchronous graph store handle (stub).
#[derive(Debug, Default, Clone)]
pub struct InfiniteDb;

impl InfiniteDb {
    /// Fetch neuron revision and type tag by raw id.
    pub fn query_neuron(&self, id: NonZeroU64) -> Option<(u64, u8)> {
        let _ = id;
        Some((1, 0))
    }

    /// Fetch subgraph revision for a root neuron.
    pub fn query_subgraph(&self, root: NonZeroU64) -> Option<u64> {
        let _ = root;
        Some(1)
    }

    /// List fiber raw ids for a neuron.
    pub fn query_fibers(&self, neuron: NonZeroU64) -> Vec<NonZeroU64> {
        let _ = neuron;
        Vec::new()
    }

    /// Persist application state; returns new revision.
    pub fn write_state(&self, neuron: NonZeroU64, _payload: &[u8]) -> Option<u64> {
        let _ = neuron;
        Some(1)
    }

    /// Persist graph definition (edit mode only).
    pub fn write_definition(&self, _root: NonZeroU64, _payload: &[u8]) -> Option<u64> {
        Some(1)
    }
}
