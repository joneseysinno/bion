//! Routing and filtering labels for behavioral domains.

use alloc::borrow::ToOwned;
use alloc::string::String;
use core::error::Error;
use core::fmt;
use core::str::FromStr;

/// Maximum label length enforced at the Soma lexical floor.
///
/// Domain-specific length policy may be stricter in Cortex.
pub const LEXICAL_MAX_LEN: usize = 256;

/// Reserved for future hierarchical labels (e.g. `parent.child`).
/// Segment-aware ordering is deferred — see [`RoutingLabel`] docs.
pub const HIERARCHY_SEPARATOR: char = '.';

/// Errors that can occur when constructing a [`RoutingLabel`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LabelError {
    /// Label is empty or whitespace-only after trimming.
    Empty,
    /// Label contains a disallowed character.
    InvalidCharacter(char),
    /// Label exceeds [`LEXICAL_MAX_LEN`].
    TooLong {
        /// Maximum permitted length in characters.
        max: usize,
    },
}

impl fmt::Display for LabelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LabelError::Empty => write!(f, "label must not be empty"),
            LabelError::InvalidCharacter(ch) => {
                write!(f, "invalid character in label: {ch:?}")
            }
            LabelError::TooLong { max } => write!(f, "label exceeds maximum length of {max}"),
        }
    }
}

impl Error for LabelError {}

/// A routing and filtering label associating a node with a behavioral domain.
///
/// Layer-neutral name — the *meaning* of a label is interpreted by Cortex;
/// Soma only defines that labeled routing metadata exists.
///
/// # Lexical floor (Soma)
/// Non-empty after trim, no whitespace, no control characters, max
/// [`LEXICAL_MAX_LEN`], charset `[alphanumeric, '-', '_']`. Stricter domain
/// policy belongs in Cortex.
///
/// # Hierarchy (deferred)
/// [`HIERARCHY_SEPARATOR`] (`.`) is reserved for future parent.child semantics.
/// Flat lexicographic ordering is intentionally not provided — segment-aware
/// comparison will be defined when hierarchy lands.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RoutingLabel(String);

impl RoutingLabel {
    /// Creates a new `RoutingLabel` after validating and normalizing the name.
    pub fn try_new(name: &str) -> Result<Self, LabelError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(LabelError::Empty);
        }
        if trimmed.len() > LEXICAL_MAX_LEN {
            return Err(LabelError::TooLong {
                max: LEXICAL_MAX_LEN,
            });
        }
        for ch in trimmed.chars() {
            if ch.is_whitespace() || ch.is_control() {
                return Err(LabelError::InvalidCharacter(ch));
            }
            if !(ch.is_alphanumeric() || ch == '-' || ch == '_') {
                return Err(LabelError::InvalidCharacter(ch));
            }
        }
        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the label as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RoutingLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.as_str())
    }
}

impl TryFrom<&str> for RoutingLabel {
    type Error = LabelError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl FromStr for RoutingLabel {
    type Err = LabelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn display_formats_hash_prefix() {
        let tag = RoutingLabel::try_new("ui").unwrap();
        assert_eq!(tag.to_string(), "#ui");
    }

    #[test]
    fn try_from_roundtrip() {
        let tag = RoutingLabel::try_from("my-domain").unwrap();
        assert_eq!(tag.as_str(), "my-domain");
    }

    #[test]
    fn empty_rejected() {
        assert_eq!(RoutingLabel::try_new(""), Err(LabelError::Empty));
        assert_eq!(RoutingLabel::try_new("   "), Err(LabelError::Empty));
    }

    #[test]
    fn invalid_character_rejected() {
        assert!(matches!(
            RoutingLabel::try_new("bad tag"),
            Err(LabelError::InvalidCharacter(' '))
        ));
    }

    #[test]
    fn trims_whitespace() {
        let tag = RoutingLabel::try_new("  valid  ").unwrap();
        assert_eq!(tag.as_str(), "valid");
    }
}
