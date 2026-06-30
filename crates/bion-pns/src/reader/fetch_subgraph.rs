//! Load a subgraph snapshot from storage via `spawn_blocking`.

use std::sync::Arc;

use bion_soma::NeuronId;

use crate::db::InfiniteDb;
use crate::reader::fetch_fibers;
use crate::reader::fetch_neuron;
use crate::types::{PnsError, SubgraphSnapshot};

/// Fetch a subgraph rooted at `root`.
pub async fn fetch_subgraph(
    db: Arc<InfiniteDb>,
    root: NeuronId,
) -> Result<SubgraphSnapshot, PnsError> {
    let raw = root.as_raw();
    let db_for_revision = Arc::clone(&db);
    let revision = tokio::task::spawn_blocking(move || db_for_revision.query_subgraph(raw))
        .await?
        .ok_or(PnsError::NotFound)?;

    let root_neuron = fetch_neuron(Arc::clone(&db), root).await?;
    let fibers = fetch_fibers(Arc::clone(&db), root).await?;

    Ok(SubgraphSnapshot {
        root,
        revision,
        neurons: vec![root_neuron],
        fibers,
    })
}
