//! bion-cns — central nervous system execution engine.
//!
//! Holds injected `PnsReader` / `PnsWriter` traits only — never touches infinite-db.

pub mod circuit;
pub mod execution;
pub mod hydration;
pub mod loops;
pub mod runtime;
pub mod types;

pub use circuit::circuit::Circuit;
pub use runtime::cns_runtime::CnsRuntime;
pub use runtime::shutdown::shutdown;
pub use runtime::start::start;
pub use types::cns_error::CnsError;
