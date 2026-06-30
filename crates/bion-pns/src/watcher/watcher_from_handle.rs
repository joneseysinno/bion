//! Consume a handle and return a boxed [`PnsWatcher`].

use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::gateway::PnsWatcher;
use crate::types::DefinitionChange;
use crate::watcher::spawn_watcher::PnsWatcherHandle;

struct ChannelWatcher {
    rx: mpsc::Receiver<DefinitionChange>,
}

#[async_trait]
impl PnsWatcher for ChannelWatcher {
    async fn recv(&mut self) -> Option<DefinitionChange> {
        self.rx.recv().await
    }
}

/// Box the receiver behind the [`PnsWatcher`] trait.
pub fn watcher_from_handle(handle: PnsWatcherHandle) -> Box<dyn PnsWatcher> {
    Box::new(ChannelWatcher { rx: handle.rx })
}
