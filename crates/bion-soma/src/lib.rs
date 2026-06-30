//! bion-soma — the molecular alphabet.
//!
//! Level 1 of the Bion stack. All types here are ontologically prior:
//! they can exist before any database, graph, or runtime is instantiated.
//!
//! # Rule
//! This crate may not import from any other `bion-*` crate.
//! It may not reference any `infinite-db` type names.
//!
//! # Purity
//! Soma contains zero effects: no I/O, no entropy, no `&mut self` state advance.
//! Identity is derived via value-threaded [`IdSeed`] / [`IdSource`].
//!
//! # `bridge` feature
//! The `bridge` feature exposes raw inner values for serialization adapters.
//! It is **intent-signaling only** — Cargo features are additive across the
//! dependency graph, so enabling `bridge` in one crate exposes accessors to
//! all consumers. Do not treat it as an enforceable access boundary. A sealed
//! trait or separate `bion-soma-bridge` crate would be required for that.

#![no_std]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod id;
pub mod neuron;
pub mod polarity;
pub mod signal;
pub mod tag;

pub use id::{
    FiberId, GanglionId, IdSeed, IdSource, NeuronId, SynapseId,
};
pub use neuron::{Arity, NeuronCapabilities, NeuronType};
pub use polarity::{Polarity, ValidSynapse};
pub use signal::{
    BoolValue, ByteBlob, FloatValue, Impulse, IntValue, SignalText, SignalType, UnitValue,
};
pub use tag::{LabelError, RoutingLabel, HIERARCHY_SEPARATOR, LEXICAL_MAX_LEN};
