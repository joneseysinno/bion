//! The only surface CNS uses to persist app state.

use async_trait::async_trait;
use bion_soma::{Impulse, NeuronId};

use crate::types::{PnsError, RevisionAck};

/// CNS calls this after Circuit calculation produces a result.
#[async_trait]
pub trait PnsWriter: Send + Sync {
    /// Persist neuron application state.
    async fn write_state(
        &self,
        neuron: NeuronId,
        impulse: Impulse,
    ) -> Result<RevisionAck, PnsError>;
}
