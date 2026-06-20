//! Signal schema, typed value atoms, and impulse payloads.

use std::fmt;

/// A boolean signal value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
/// Does not implement [`Eq`] — use [`PartialEq`] only. Equality uses
/// `f64::to_bits()` so NaN == NaN (deterministic value comparison).
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FloatValue(f64);

impl FloatValue {
    /// Wraps a floating-point payload.
    pub const fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the inner float (bridge / serialization only).
    #[cfg(feature = "bridge")]
    pub const fn as_f64(self) -> f64 {
        self.0
    }
}

impl PartialEq for FloatValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl fmt::Display for FloatValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A UTF-8 text signal value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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
/// `SignalType` is the type system of the Bion graph. Two Fibers may only
/// form a valid Synapse if their SignalTypes are compatible. Compatibility
/// checking lives in `bion-cortex` (the `Immune` validator) — this enum
/// only defines what types exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// Event-only trigger with no payload.
    Unit,
}

/// Why two signal types are or are not compatible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CompatibilityReason {
    /// Types differ with no lossless conversion path.
    TypeMismatch,
    /// Conversion would lose information.
    LossyConversion,
}

/// Result of comparing two signal types for lossless compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Compatibility {
    /// Same type — exact match.
    Exact,
    /// Lossless widening (e.g. Int → Float).
    Widening {
        /// Source schema type.
        from: SignalType,
        /// Target schema type.
        to: SignalType,
    },
    /// Incompatible — coercion would lose information or change meaning.
    Incompatible {
        /// Source schema type.
        from: SignalType,
        /// Target schema type.
        to: SignalType,
        /// Why the types are incompatible.
        reason: CompatibilityReason,
    },
}

impl SignalType {
    /// Returns the compatibility relationship between `self` and `target`.
    ///
    /// This is intentionally conservative: only lossless promotions.
    /// `Int` → `Float` is allowed (widening). `Float` → `Int` is not
    /// (truncation). `Text` → `Bytes` is not (encoding ambiguity).
    pub fn compatibility_with(self, target: SignalType) -> Compatibility {
        if self == target {
            return Compatibility::Exact;
        }
        match (self, target) {
            (SignalType::Int, SignalType::Float) => Compatibility::Widening {
                from: SignalType::Int,
                to: SignalType::Float,
            },
            _ => Compatibility::Incompatible {
                from: self,
                to: target,
                reason: CompatibilityReason::TypeMismatch,
            },
        }
    }

    /// Convenience predicate — true when [`compatibility_with`](Self::compatibility_with) is not [`Compatibility::Incompatible`].
    pub fn is_compatible_with(self, target: SignalType) -> bool {
        !matches!(
            self.compatibility_with(target),
            Compatibility::Incompatible { .. }
        )
    }
}

/// A typed data quantum — the payload carried by a Synapse.
///
/// `Impulse` is the *value* that corresponds to a [`SignalType`] *schema*.
/// Every variant wraps a value newtype that corresponds to exactly one [`SignalType`].
#[derive(Debug, Clone, PartialEq)]
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
    /// Unit trigger payload.
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

    /// Returns the compatibility relationship with the given target schema.
    pub fn compatibility_with(&self, target: SignalType) -> Compatibility {
        self.signal_type().compatibility_with(target)
    }

    /// Convenience predicate — true when [`compatibility_with`](Self::compatibility_with) is not [`Compatibility::Incompatible`].
    pub fn is_compatible_with(&self, target: SignalType) -> bool {
        self.signal_type().is_compatible_with(target)
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
            Impulse::Float(FloatValue::new(1.5)).signal_type(),
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
    fn compatibility_exact_and_widening() {
        assert_eq!(
            SignalType::Int.compatibility_with(SignalType::Int),
            Compatibility::Exact
        );
        assert_eq!(
            SignalType::Int.compatibility_with(SignalType::Float),
            Compatibility::Widening {
                from: SignalType::Int,
                to: SignalType::Float,
            }
        );
    }

    #[test]
    fn compatibility_incompatible_float_to_int() {
        assert_eq!(
            SignalType::Float.compatibility_with(SignalType::Int),
            Compatibility::Incompatible {
                from: SignalType::Float,
                to: SignalType::Int,
                reason: CompatibilityReason::TypeMismatch,
            }
        );
        assert!(!SignalType::Float.is_compatible_with(SignalType::Int));
    }

    #[test]
    fn impulse_delegates_compatibility() {
        let impulse = Impulse::Int(IntValue::new(1));
        assert!(impulse.is_compatible_with(SignalType::Float));
        assert!(!impulse.is_compatible_with(SignalType::Text));
    }

    #[test]
    fn float_value_nan_equals_nan() {
        let a = FloatValue::new(f64::NAN);
        let b = FloatValue::new(f64::NAN);
        assert_eq!(a, b);
    }
}
