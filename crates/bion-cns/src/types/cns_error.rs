//! CNS error surface.

use core::fmt;

/// Errors raised during circuit execution and hydration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CnsError {
    /// Hydration could not load the requested subgraph.
    Hydration(String),
    /// Execution rejected the impulse.
    Execution(String),
    /// Channel closed unexpectedly.
    ChannelClosed,
    /// PNS gateway returned an error.
    Pns(String),
}

impl fmt::Display for CnsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hydration(msg) => write!(f, "hydration: {msg}"),
            Self::Execution(msg) => write!(f, "execution: {msg}"),
            Self::ChannelClosed => write!(f, "channel closed"),
            Self::Pns(msg) => write!(f, "pns: {msg}"),
        }
    }
}

impl std::error::Error for CnsError {}

impl From<bion_pns::PnsError> for CnsError {
    fn from(err: bion_pns::PnsError) -> Self {
        Self::Pns(err.to_string())
    }
}
