//! Frontend-facing transport adapters.

#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "ws")]
pub mod ws;
