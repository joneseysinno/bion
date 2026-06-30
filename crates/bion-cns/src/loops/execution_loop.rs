//! Impulse processing loop.

use std::sync::Arc;

use bion_pns::{ImpulseEnvelope, PnsWriter, StateUpdate};
use tokio::sync::{mpsc, RwLock};

use crate::circuit::circuit::Circuit;
use crate::execution::executor::Executor;

/// Receives impulses, executes the circuit, persists state, and emits updates.
pub async fn run_execution_loop(
    circuit: Arc<RwLock<Circuit>>,
    mut impulse_rx: mpsc::Receiver<ImpulseEnvelope>,
    pns_writer: Arc<dyn PnsWriter>,
    result_tx: mpsc::Sender<StateUpdate>,
) {
    let executor = Executor::new(Arc::clone(&circuit));
    while let Some(envelope) = impulse_rx.recv().await {
        let target = envelope.target;
        let impulse = envelope.impulse.clone();
        match executor.execute(target, envelope.impulse).await {
            Ok(result) => {
                if let Ok(ack) = pns_writer.write_state(target, result.clone()).await {
                    let update = StateUpdate {
                        source: target,
                        impulse: result,
                        revision: ack.revision,
                    };
                    let _ = result_tx.try_send(update);
                }
            }
            Err(_) => {
                let _ = pns_writer.write_state(target, impulse).await;
            }
        }
    }
}
