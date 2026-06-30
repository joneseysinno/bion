//! List fibers for a neuron via `spawn_blocking`.

use std::sync::Arc;

use bion_soma::{FiberId, NeuronId, Polarity, SignalType};

use crate::db::InfiniteDb;
use crate::types::{FiberDescriptor, PnsError};

/// Fetch all fiber descriptors for `neuron`.
pub async fn fetch_fibers(
    db: Arc<InfiniteDb>,
    neuron: NeuronId,
) -> Result<Vec<FiberDescriptor>, PnsError> {
    let neuron_raw = neuron.as_raw();
    let fibers = tokio::task::spawn_blocking(move || db.query_fibers(neuron_raw))
        .await?;

    Ok(fibers
        .into_iter()
        .map(|raw| FiberDescriptor {
            id: FiberId::from_raw(raw),
            neuron,
            signal_type: SignalType::Unit,
            polarity: Polarity::Afferent,
        })
        .collect())
}
