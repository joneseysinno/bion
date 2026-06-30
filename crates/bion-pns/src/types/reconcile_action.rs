//! UI/DB drift reconciliation actions.

/// Action taken when client revision lags storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReconcileAction {
    /// Push a corrective state update to subscribers.
    PushCorrective {
        /// Revision observed in storage.
        storage_revision: u64,
    },
    /// No action required — revisions are in sync.
    NoOp,
}
