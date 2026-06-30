//! Definition-change hot-reload loop.

use std::sync::Arc;

use bion_pns::PnsReader;
use bion_pns::PnsWatcher;
use tokio::sync::RwLock;

use crate::circuit::circuit::Circuit;
use crate::hydration::hydrate_subgraph::hydrate_subgraph;
use crate::hydration::rebuild_fragment::rebuild_fragment;

/// Watches for definition changes and hot-reloads circuit fragments.
pub async fn run_watch_loop(
    circuit: Arc<RwLock<Circuit>>,
    mut watcher: Box<dyn PnsWatcher>,
    pns_reader: Arc<dyn PnsReader>,
) {
    while let Some(change) = watcher.recv().await {
        if let Ok(fragment) = hydrate_subgraph(Arc::clone(&pns_reader), change.subgraph_id).await {
            let mut guard = circuit.write().await;
            rebuild_fragment(&mut guard, fragment);
        }
    }
}
