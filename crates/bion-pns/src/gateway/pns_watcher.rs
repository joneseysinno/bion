//! Definition-change notifications for CNS watch loop.

use async_trait::async_trait;

use crate::types::DefinitionChange;

/// Delivers [`DefinitionChange`] notifications to CNS.
/// Backed by `tokio::sync::mpsc::Receiver`.
#[async_trait]
pub trait PnsWatcher: Send {
    /// Wait for the next definition change, or `None` when the channel closes.
    async fn recv(&mut self) -> Option<DefinitionChange>;
}
