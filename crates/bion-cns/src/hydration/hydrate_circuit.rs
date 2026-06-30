//! Load the root circuit from PNS.

use std::sync::Arc;

use bion_pns::PnsReader;
use bion_soma::NeuronId;

use crate::circuit::circuit::Circuit;
use crate::hydration::hydrate_subgraph::hydrate_subgraph;
use crate::types::CnsError;

/// Hydrate the full circuit for `root` via [`PnsReader::fetch_subgraph`].
pub async fn hydrate_circuit(
    reader: Arc<dyn PnsReader>,
    root: NeuronId,
) -> Result<Circuit, CnsError> {
    let fragment = hydrate_subgraph(reader, root).await?;
    Ok(Circuit {
        root: fragment.root,
        ganglia: fragment.ganglia,
        synapses: fragment.synapses,
    })
}
