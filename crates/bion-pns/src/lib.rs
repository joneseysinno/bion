//! bion-pns — peripheral nervous system.
//!
//! Owns all contact with infinite-db. Exposes gateway traits for CNS and
//! transport adapters for frontends.

pub mod db;
pub mod gateway;
pub mod reader;
pub mod transport;
pub mod types;
pub mod watcher;
pub mod writer;

pub use gateway::{PnsReader, PnsWatcher, PnsWriter};
pub use types::{
    DefinitionChange, FiberDescriptor, ImpulseEnvelope, NeuronDescriptor, PnsError,
    ReconcileAction, RevisionAck, StateUpdate, SubgraphSnapshot,
};
