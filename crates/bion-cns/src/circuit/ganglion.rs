//! Ganglion identity (wraps soma `GanglionId`).

pub use bion_soma::GanglionId;

/// Static ganglion definition in a circuit blueprint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ganglion {
    /// Ganglion identity.
    pub id: GanglionId,
}

/// Live ganglion instance holding runtime state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GanglionInstance {
    /// Ganglion identity.
    pub id: GanglionId,
    /// Last impulse held by this ganglion (if any).
    pub held: Option<bion_soma::Impulse>,
}
