//! Drives a [`Circuit`] through signal wavefronts.

use std::sync::Arc;
use tokio::sync::RwLock;

use bion_soma::{Impulse, NeuronId};

use crate::circuit::circuit::Circuit;
use crate::types::CnsError;

/// Execution engine for a live circuit.
pub struct Executor {
    circuit: Arc<RwLock<Circuit>>,
}

impl Executor {
    /// Wrap a shared circuit.
    pub fn new(circuit: Arc<RwLock<Circuit>>) -> Self {
        Self { circuit }
    }

    /// Run one impulse through the circuit.
    pub async fn execute(&self, target: NeuronId, impulse: Impulse) -> Result<Impulse, CnsError> {
        let mut circuit = self.circuit.write().await;
        circuit.apply_impulse(target, impulse)
    }
}
