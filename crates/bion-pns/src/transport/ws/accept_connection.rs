//! Accept a WebSocket connection (stub until full WS wiring).

use crate::types::PnsError;

/// Placeholder accept hook — returns transport error until wired.
pub async fn accept_connection(_addr: &str) -> Result<(), PnsError> {
    Err(PnsError::Transport("websocket accept not yet implemented".into()))
}
