//! Start the three concurrent CNS loops.

use std::sync::Arc;

use bion_pns::{ImpulseEnvelope, PnsReader, PnsWatcher, PnsWriter, StateUpdate};
use bion_soma::NeuronId;
use tokio::sync::{mpsc, RwLock};

use crate::circuit::circuit::Circuit;
use crate::hydration::hydrate_circuit::hydrate_circuit;
use crate::loops::{run_execution_loop, run_output_loop, run_watch_loop};
use crate::runtime::cns_runtime::CnsRuntime;
use crate::types::CnsError;

/// Hydrate the circuit and spawn execution, watch, and output loops.
pub async fn start(
    reader: Arc<dyn PnsReader>,
    writer: Arc<dyn PnsWriter>,
    watcher: Box<dyn PnsWatcher>,
    root: NeuronId,
    impulse_rx: mpsc::Receiver<ImpulseEnvelope>,
) -> Result<(CnsRuntime, Arc<RwLock<Circuit>>, Arc<RwLock<Vec<mpsc::Sender<StateUpdate>>>>), CnsError>
{
    let circuit = Arc::new(RwLock::new(hydrate_circuit(Arc::clone(&reader), root).await?));
    let (result_tx, result_rx) = mpsc::channel(256);
    let subscribers = Arc::new(RwLock::new(Vec::<mpsc::Sender<StateUpdate>>::new()));

    let execution = tokio::spawn(run_execution_loop(
        Arc::clone(&circuit),
        impulse_rx,
        writer,
        result_tx,
    ));

    let watch = tokio::spawn(run_watch_loop(
        Arc::clone(&circuit),
        watcher,
        reader,
    ));

    let output = tokio::spawn(run_output_loop(result_rx, Arc::clone(&subscribers)));

    Ok((CnsRuntime { execution, watch, output }, circuit, subscribers))
}
