//! Load one subgraph fragment from PNS.

use std::sync::Arc;

use bion_pns::PnsReader;
use bion_soma::NeuronId;

use crate::hydration::snapshot::snapshot_to_fragment;
use crate::types::{CircuitFragment, CnsError};

/// Fetch and translate a subgraph into a [`CircuitFragment`].
pub async fn hydrate_subgraph(
    reader: Arc<dyn PnsReader>,
    id: NeuronId,
) -> Result<CircuitFragment, CnsError> {
    let snapshot = reader.fetch_subgraph(id).await?;
    Ok(snapshot_to_fragment(&snapshot))
}
