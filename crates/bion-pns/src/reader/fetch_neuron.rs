//! Load a single neuron from storage via `spawn_blocking`.

use std::sync::Arc;

use bion_soma::{NeuronId, NeuronType};

use crate::db::InfiniteDb;
use crate::types::{NeuronDescriptor, PnsError};

/// Fetch one neuron descriptor by id.
pub async fn fetch_neuron(db: Arc<InfiniteDb>, id: NeuronId) -> Result<NeuronDescriptor, PnsError> {
    let raw = id.as_raw();
    tokio::task::spawn_blocking(move || {
        db.query_neuron(raw)
            .map(|(revision, _tag)| NeuronDescriptor {
                id,
                neuron_type: NeuronType::Interneuron,
                revision,
            })
            .ok_or(PnsError::NotFound)
    })
    .await?
}
