//! Sequences ganglion execution order.

use bion_soma::GanglionId;

/// Orders ganglia for sequential or wavefront execution.
#[derive(Debug, Default, Clone)]
pub struct Pacemaker {
    /// Ordered ganglion ids.
    pub order: Vec<GanglionId>,
}
