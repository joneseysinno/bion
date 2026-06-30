//! Read an impulse from a WebSocket stream (stub).

use crate::types::{ImpulseEnvelope, PnsError};

/// Placeholder read hook.
pub async fn read_impulse(_stream: ()) -> Result<ImpulseEnvelope, PnsError> {
    Err(PnsError::Transport("websocket read not yet implemented".into()))
}
