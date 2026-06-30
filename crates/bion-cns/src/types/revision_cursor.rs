//! Tracks last-written and last-seen revisions per neuron.

use bion_soma::NeuronId;
use std::collections::HashMap;

/// Per-neuron revision cursor for optimistic UI reconciliation.
#[derive(Debug, Default, Clone)]
pub struct RevisionCursor {
    written: HashMap<NeuronId, u64>,
    seen: HashMap<NeuronId, u64>,
}

impl RevisionCursor {
    /// Record a successful state write revision.
    pub fn record_written(&mut self, neuron: NeuronId, revision: u64) {
        self.written.insert(neuron, revision);
    }

    /// Record a revision observed from storage or a subscriber update.
    pub fn record_seen(&mut self, neuron: NeuronId, revision: u64) {
        self.seen.insert(neuron, revision);
    }

    /// Last revision written for `neuron`.
    pub fn last_written(&self, neuron: NeuronId) -> Option<u64> {
        self.written.get(&neuron).copied()
    }

    /// Last revision seen for `neuron`.
    pub fn last_seen(&self, neuron: NeuronId) -> Option<u64> {
        self.seen.get(&neuron).copied()
    }

    /// True when another writer has advanced storage beyond our cursor.
    pub fn needs_reconciliation(&self, neuron: NeuronId) -> bool {
        match (self.last_written(neuron), self.last_seen(neuron)) {
            (Some(w), Some(s)) => s > w.saturating_add(1),
            _ => false,
        }
    }
}
