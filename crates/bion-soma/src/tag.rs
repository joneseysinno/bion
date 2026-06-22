//! Routing and filtering labels for behavioral domains.

use alloc::borrow::ToOwned;
use alloc::string::String;
use core::error::Error;
use core::fmt;
use core::str::FromStr;

/// Errors that can occur when constructing a [`CortexTag`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TagError {
    /// Tag name is empty or whitespace-only after trimming.
    Empty,
    /// Tag name contains a disallowed character.
    InvalidCharacter(char),
    /// Tag name exceeds the maximum allowed length.
    TooLong {
        /// Maximum permitted length in characters.
        max: usize,
    },
}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagError::Empty => write!(f, "tag name must not be empty"),
            TagError::InvalidCharacter(ch) => {
                write!(f, "invalid character in tag name: {ch:?}")
            }
            TagError::TooLong { max } => write!(f, "tag name exceeds maximum length of {max}"),
        }
    }
}

impl Error for TagError {}

/// A routing and filtering label associating a node with a behavioral domain.
///
/// `CortexTag` is inert data — it carries no behavior of its own.
/// The Cortex layer reads these tags to know which rules apply to a given
/// Neuron or Fiber.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CortexTag(String);

impl CortexTag {
    /// Creates a new `CortexTag` after validating and normalizing the name.
    pub fn try_new(name: &str) -> Result<Self, TagError> {
        const MAX_LEN: usize = 256;
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(TagError::Empty);
        }
        if trimmed.len() > MAX_LEN {
            return Err(TagError::TooLong { max: MAX_LEN });
        }
        for ch in trimmed.chars() {
            if !(ch.is_alphanumeric() || ch == '-' || ch == '_') {
                return Err(TagError::InvalidCharacter(ch));
            }
        }
        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the tag name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CortexTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.as_str())
    }
}

impl TryFrom<&str> for CortexTag {
    type Error = TagError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl FromStr for CortexTag {
    type Err = TagError;

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
        let tag = CortexTag::try_new("ui").unwrap();
        assert_eq!(tag.to_string(), "#ui");
    }

    #[test]
    fn try_from_roundtrip() {
        let tag = CortexTag::try_from("my-domain").unwrap();
        assert_eq!(tag.as_str(), "my-domain");
    }

    #[test]
    fn empty_rejected() {
        assert_eq!(CortexTag::try_new(""), Err(TagError::Empty));
        assert_eq!(CortexTag::try_new("   "), Err(TagError::Empty));
    }

    #[test]
    fn invalid_character_rejected() {
        assert!(matches!(
            CortexTag::try_new("bad tag"),
            Err(TagError::InvalidCharacter(' '))
        ));
    }

    #[test]
    fn trims_whitespace() {
        let tag = CortexTag::try_new("  valid  ").unwrap();
        assert_eq!(tag.as_str(), "valid");
    }
}
