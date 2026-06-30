//! Signal schema, typed value atoms, and impulse payloads.

use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};

/// A boolean signal value.
///
/// Deliberately-unconstrained nominal wrapper — carries no invariant beyond
/// distinguishing bool payloads from other signal types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoolValue(bool);

impl BoolValue {
    /// Wraps a boolean payload.
    pub const fn new(value: bool) -> Self {
        Self(value)
    }

    /// Returns the inner boolean (bridge / serialization only).
    #[cfg(feature = "bridge")]
    pub const fn as_bool(self) -> bool {
        self.0
    }
}

impl fmt::Display for BoolValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A 64-bit signed integer signal value.
///
/// Deliberately-unconstrained nominal wrapper — carries no invariant beyond
/// distinguishing integer payloads from other signal types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntValue(i64);

impl IntValue {
    /// Wraps an integer payload.
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Returns the inner integer (bridge / serialization only).
    #[cfg(feature = "bridge")]
    pub const fn as_i64(self) -> i64 {
        self.0
    }
}

impl fmt::Display for IntValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A 64-bit IEEE 754 floating-point signal value.
///
/// Only finite values are representable. `NaN` and `±Inf` are rejected at
/// construction — a signal value of non-finite magnitude has no transducer
/// meaning in Soma.
///
/// Implements [`Eq`], [`Hash`], and [`Ord`] via `f64::total_cmp` over finite
/// values (`-0.0 < +0.0`).
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FloatValue(f64);

impl FloatValue {
    /// Wraps a finite floating-point payload.
    ///
    /// Returns `None` for `NaN`, `+Inf`, and `-Inf`.
    pub fn new(value: f64) -> Option<Self> {
        if value.is_finite() {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the inner float (bridge / serialization only).
    #[cfg(feature = "bridge")]
    pub const fn as_f64(self) -> f64 {
        self.0
    }
}

impl PartialEq for FloatValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0) == Ordering::Equal
    }
}

impl Eq for FloatValue {}

impl PartialOrd for FloatValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FloatValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Hash for FloatValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl fmt::Display for FloatValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A UTF-8 text signal value.
///
/// Deliberately-unconstrained nominal wrapper — domain-specific text policy
/// (max length, charset) belongs in Cortex.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignalText(String);

impl SignalText {
    /// Wraps a UTF-8 text payload.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the text as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the wrapper and returns the inner string (bridge / serialization only).
    #[cfg(feature = "bridge")]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for SignalText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

/// Raw binary signal data. No encoding assumed.
///
/// Deliberately-unconstrained nominal wrapper — encoding contracts belong in
/// Cortex or the Membrane layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ByteBlob(Vec<u8>);

impl ByteBlob {
    /// Wraps a byte payload.
    pub fn new(value: impl Into<Vec<u8>>) -> Self {
        Self(value.into())
    }

    /// Returns the number of bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true when the blob is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Consumes the wrapper and returns the inner bytes (bridge / serialization only).
    #[cfg(feature = "bridge")]
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl fmt::Display for ByteBlob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} bytes]", self.len())
    }
}

/// Explicit trigger token — the signal is the fact of firing, not data.
///
/// # What `Unit` is not
/// - **Not silence** — silence (did not fire) is modeled as `Option<Impulse>`
///   at the Connectome edge. Absence is the type system's job.
/// - **Not inhibition** — active suppression is future work via the reserved
///   `PolaritySign` / `Charge` dipole model (deferred).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnitValue;

impl UnitValue {
    /// Returns the unit trigger token.
    pub const fn new() -> Self {
        Self
    }
}

impl fmt::Display for UnitValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("()")
    }
}

/// The type of data that a Fiber carries and a Synapse transmits.
///
/// `SignalType` is the type system of the Bion graph — it defines what types
/// exist. Whether two types may connect, widen, or coerce is decided in
/// `bion-cortex` (`Immune`, `Morphogen`), not here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SignalType {
    /// A boolean signal: true or false, on or off, fired or silent.
    Bool,
    /// A 64-bit signed integer.
    Int,
    /// A 64-bit IEEE 754 floating-point number (marker — no payload).
    Float,
    /// A UTF-8 string of arbitrary length.
    Text,
    /// Raw binary data. No encoding assumed.
    Bytes,
    /// Event-only trigger with no payload (fired, not silence).
    Unit,
}

/// A typed data quantum — the payload carried by a Synapse.
///
/// `Impulse` is the *value* that corresponds to a [`SignalType`] *schema*.
/// Every variant wraps a value newtype that corresponds to exactly one [`SignalType`].
///
/// # Silence at the Connectome boundary
/// A neuron that did not fire is represented as `Option<Impulse>::None` at the
/// Connectome layer — not as a variant of `Impulse`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Impulse {
    /// Boolean payload.
    Bool(BoolValue),
    /// Integer payload.
    Int(IntValue),
    /// Floating-point payload.
    Float(FloatValue),
    /// Text payload.
    Text(SignalText),
    /// Binary payload.
    Bytes(ByteBlob),
    /// Unit trigger payload (fired with no data — not silence).
    Unit(UnitValue),
}

impl Impulse {
    /// Returns the [`SignalType`] of this impulse.
    pub const fn signal_type(&self) -> SignalType {
        match self {
            Impulse::Bool(_) => SignalType::Bool,
            Impulse::Int(_) => SignalType::Int,
            Impulse::Float(_) => SignalType::Float,
            Impulse::Text(_) => SignalType::Text,
            Impulse::Bytes(_) => SignalType::Bytes,
            Impulse::Unit(_) => SignalType::Unit,
        }
    }
}

impl fmt::Display for Impulse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Impulse::Bool(v) => write!(f, "Bool({v})"),
            Impulse::Int(v) => write!(f, "Int({v})"),
            Impulse::Float(v) => write!(f, "Float({v})"),
            Impulse::Text(v) => write!(f, "Text({v})"),
            Impulse::Bytes(v) => write!(f, "Bytes({v})"),
            Impulse::Unit(_) => write!(f, "Unit"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn impulse_signal_type_variants() {
        assert_eq!(
            Impulse::Bool(BoolValue::new(true)).signal_type(),
            SignalType::Bool
        );
        assert_eq!(
            Impulse::Int(IntValue::new(42)).signal_type(),
            SignalType::Int
        );
        assert_eq!(
            Impulse::Float(FloatValue::new(1.5).unwrap()).signal_type(),
            SignalType::Float
        );
        assert_eq!(
            Impulse::Text(SignalText::new("hi")).signal_type(),
            SignalType::Text
        );
        assert_eq!(
            Impulse::Bytes(ByteBlob::new([1u8, 2])).signal_type(),
            SignalType::Bytes
        );
        assert_eq!(
            Impulse::Unit(UnitValue::new()).signal_type(),
            SignalType::Unit
        );
    }

    #[test]
    fn float_value_rejects_non_finite() {
        assert!(FloatValue::new(f64::NAN).is_none());
        assert!(FloatValue::new(f64::INFINITY).is_none());
        assert!(FloatValue::new(f64::NEG_INFINITY).is_none());
        assert!(FloatValue::new(1.5).is_some());
    }

    #[test]
    fn float_value_orders_zero_signs() {
        let neg = FloatValue::new(-0.0).unwrap();
        let pos = FloatValue::new(0.0).unwrap();
        assert!(neg < pos);
        assert_ne!(neg, pos);
    }

    #[test]
    fn float_value_eq_is_lawful() {
        let a = FloatValue::new(1.5).unwrap();
        let b = FloatValue::new(1.5).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn impulse_orders_distinct_variants() {
        let mut impulses = [
            Impulse::Int(IntValue::new(1)),
            Impulse::Float(FloatValue::new(2.0).unwrap()),
        ];
        impulses.sort();
        assert_ne!(impulses[0], impulses[1]);
    }
}
