//! State update broadcast loop.

use std::sync::Arc;

use bion_pns::StateUpdate;
use tokio::sync::{mpsc, RwLock};

/// Broadcasts state updates to subscribers; drops on backpressure.
pub async fn run_output_loop(
    mut result_rx: mpsc::Receiver<StateUpdate>,
    subscribers: Arc<RwLock<Vec<mpsc::Sender<StateUpdate>>>>,
) {
    while let Some(update) = result_rx.recv().await {
        let subs = subscribers.read().await;
        subs.iter().for_each(|tx| {
            let _ = tx.try_send(update.clone());
        });
    }
}
