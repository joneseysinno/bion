//! Graceful shutdown of a running CNS.

use crate::runtime::cns_runtime::CnsRuntime;

/// Abort all loop tasks.
pub async fn shutdown(runtime: CnsRuntime) {
    runtime.execution.abort();
    runtime.watch.abort();
    runtime.output.abort();
    let _ = runtime.execution.await;
    let _ = runtime.watch.await;
    let _ = runtime.output.await;
}
