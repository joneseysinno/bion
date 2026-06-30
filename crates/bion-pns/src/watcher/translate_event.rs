//! Map a storage derivation event to a PNS definition change.

use bion_soma::NeuronId;

use crate::types::DefinitionChange;

/// Raw storage event (revision + root id) before channel delivery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DbDerivationEvent {
    /// Changed subgraph root (raw u64).
    pub root_raw: u64,
    /// New storage revision.
    pub revision: u64,
}

/// Translate a db derivation bus event into a [`DefinitionChange`].
pub fn translate_event(event: DbDerivationEvent) -> Option<DefinitionChange> {
    let root = core::num::NonZeroU64::new(event.root_raw)?;
    Some(DefinitionChange {
        subgraph_id: NeuronId::from_raw(root),
        revision: event.revision,
    })
}
