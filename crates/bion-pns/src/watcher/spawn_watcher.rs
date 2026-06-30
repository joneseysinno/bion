//! Bridge DerivationBus notifications into an mpsc channel.

use tokio::sync::mpsc;

use crate::types::DefinitionChange;
use crate::watcher::translate_event::{translate_event, DbDerivationEvent};

/// Handle wrapping the receiver side of a definition-change channel.
pub struct PnsWatcherHandle {
    pub(crate) rx: mpsc::Receiver<DefinitionChange>,
}

/// Spawn a watcher task that forwards translated db events into a channel.
pub fn spawn_watcher(
    mut event_rx: mpsc::Receiver<DbDerivationEvent>,
) -> (PnsWatcherHandle, tokio::task::JoinHandle<()>) {
    let (tx, rx) = mpsc::channel(64);
    let handle = tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            if let Some(change) = translate_event(event) {
                let _ = tx.try_send(change);
            }
        }
    });
    (PnsWatcherHandle { rx }, handle)
}
