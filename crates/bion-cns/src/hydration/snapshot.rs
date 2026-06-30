//! Translate a PNS subgraph snapshot into a circuit fragment.

use bion_pns::SubgraphSnapshot;
use bion_soma::IdSeed;

use crate::circuit::ganglion::GanglionInstance;
use crate::types::CircuitFragment;

/// Convert a storage snapshot into an executable fragment.
pub fn snapshot_to_fragment(snapshot: &SubgraphSnapshot) -> CircuitFragment {
    let ganglia: Vec<GanglionInstance> = (0..snapshot.neurons.len().max(1))
        .filter_map(|_| {
            IdSeed::first()
                .mint_ganglion()
                .map(|(id, _)| GanglionInstance { id, held: None })
        })
        .collect();

    CircuitFragment {
        root: Some(snapshot.root),
        ganglia,
        synapses: Vec::new(),
    }
}
