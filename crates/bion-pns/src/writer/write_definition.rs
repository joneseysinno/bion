//! Persist graph definition via `spawn_blocking` (edit mode only).

use std::sync::Arc;

use bion_soma::NeuronId;

use crate::db::InfiniteDb;
use crate::types::{PnsError, RevisionAck};

/// Write a subgraph definition to storage.
pub async fn write_definition(
    db: Arc<InfiniteDb>,
    root: NeuronId,
    payload: &[u8],
) -> Result<RevisionAck, PnsError> {
    let raw = root.as_raw();
    let payload = payload.to_vec();
    let revision = tokio::task::spawn_blocking(move || db.write_definition(raw, &payload))
        .await?
        .ok_or(PnsError::NotFound)?;
    Ok(RevisionAck { revision })
}
