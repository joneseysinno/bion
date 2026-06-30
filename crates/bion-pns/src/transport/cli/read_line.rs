//! Read one impulse line from stdin.

use std::io::{self, BufRead};

use bion_soma::{BoolValue, Impulse, NeuronId};

use crate::types::{ImpulseEnvelope, PnsError};

/// Parse `target_id:bool:value` (or JSON in future) into an [`ImpulseEnvelope`].
pub fn read_line(stdin: io::Stdin) -> Result<ImpulseEnvelope, PnsError> {
    let mut line = String::new();
    stdin
        .lock()
        .read_line(&mut line)
        .map_err(|e| PnsError::Transport(e.to_string()))?;
    let line = line.trim();
    if line.is_empty() {
        return Err(PnsError::Codec("empty line".into()));
    }
    let (id_part, value_part) = line
        .split_once(':')
        .ok_or_else(|| PnsError::Codec("expected target_id:value".into()))?;
    let id_raw: u64 = id_part
        .parse()
        .map_err(|_| PnsError::Codec("invalid neuron id".into()))?;
    let nz = core::num::NonZeroU64::new(id_raw).ok_or(PnsError::Codec("zero neuron id".into()))?;
    let target = NeuronId::from_raw(nz);
    let value: bool = value_part
        .parse()
        .map_err(|_| PnsError::Codec("invalid bool value".into()))?;
    Ok(ImpulseEnvelope {
        target,
        impulse: Impulse::Bool(BoolValue::new(value)),
        client_revision: None,
    })
}
