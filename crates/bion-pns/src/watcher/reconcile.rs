//! Compare UI and storage revisions.

use crate::types::ReconcileAction;

/// Reconcile client revision against storage revision.
pub fn reconcile(ui_rev: u64, db_rev: u64) -> ReconcileAction {
    if db_rev > ui_rev.saturating_add(1) {
        ReconcileAction::PushCorrective {
            storage_revision: db_rev,
        }
    } else {
        ReconcileAction::NoOp
    }
}
