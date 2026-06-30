//! Write a state update as human-readable text to stdout.

use std::io::{self, Write};

use crate::types::{PnsError, StateUpdate};

/// Format and write a [`StateUpdate`] line.
pub fn write_output(mut stdout: io::Stdout, update: &StateUpdate) -> Result<(), PnsError> {
    writeln!(
        stdout,
        "{} rev={} {}",
        update.source, update.revision, update.impulse
    )
    .map_err(|e| PnsError::Transport(e.to_string()))
}
