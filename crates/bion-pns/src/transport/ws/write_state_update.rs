//! Push a state update over WebSocket (stub).

use crate::types::{PnsError, StateUpdate};

/// Placeholder write hook.
pub async fn write_state_update(_stream: (), update: &StateUpdate) -> Result<(), PnsError> {
    let _ = update;
    Err(PnsError::Transport("websocket write not yet implemented".into()))
}
