//! Persist application state via `spawn_blocking`.

use std::sync::Arc;

use bion_soma::{Impulse, NeuronId};

use crate::db::InfiniteDb;
use crate::types::{PnsError, RevisionAck};

/// Write neuron state to storage.
pub async fn write_state(
    db: Arc<InfiniteDb>,
    neuron: NeuronId,
    impulse: Impulse,
) -> Result<RevisionAck, PnsError> {
    let raw = neuron.as_raw();
    let _ = impulse;
    let revision = tokio::task::spawn_blocking(move || db.write_state(raw, &[]))
        .await?
        .ok_or(PnsError::NotFound)?;
    Ok(RevisionAck { revision })
}
