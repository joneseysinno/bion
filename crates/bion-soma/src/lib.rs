//! bion-soma — the molecular alphabet.
//!
//! Level 1 of the Bion stack. All types here are ontologically prior:
//! they can exist before any database, graph, or runtime is instantiated.
//!
//! # Rule
//! This crate may not import from any other `bion-*` crate.
//! It may not reference any `infinite-db` type names.

#![no_std]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::mutex_atomic)]

extern crate alloc;

pub mod id;
pub mod neuron;
pub mod polarity;
pub mod signal;
pub mod tag;

pub use id::{FiberId, IdSeed, IdSource, NeuronId};
pub use neuron::{NeuronCapabilities, NeuronType};
pub use polarity::{Polarity, ValidSynapse};
pub use signal::{
    BoolValue, ByteBlob, Compatibility, CompatibilityReason, FloatValue, Impulse, IntValue,
    SignalText, SignalType, UnitValue,
};
pub use tag::{CortexTag, TagError};
