//! Assembled CNS runtime with join handles for the three loops.

use tokio::task::JoinHandle;

/// Live runtime — holds task handles for graceful shutdown.
pub struct CnsRuntime {
    /// Execution loop task.
    pub execution: JoinHandle<()>,
    /// Watch loop task.
    pub watch: JoinHandle<()>,
    /// Output loop task.
    pub output: JoinHandle<()>,
}
