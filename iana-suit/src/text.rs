//! SUIT Text Values (IANA registry: suit-text-values).
//!
//! Reference: [RFC-ietf-suit-manifest-34](https://www.iana.org/go/draft-ietf-suit-manifest-34)


/// Manifest Description.
const MANIFEST_DESCRIPTION: i32 = 1;
/// Update Description.
const UPDATE_DESCRIPTION: i32 = 2;
/// Manifest JSON Source.
const MANIFEST_JSON_SOURCE: i32 = 3;
/// Manifest YAML Source.
const MANIFEST_YAML_SOURCE: i32 = 4;

/// A SUIT Text Value label.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TextValue(i32);

impl TextValue {
    /// Manifest Description.
    pub const MANIFEST_DESCRIPTION: Self = Self(MANIFEST_DESCRIPTION);
    /// Update Description.
    pub const UPDATE_DESCRIPTION: Self = Self(UPDATE_DESCRIPTION);
    /// Manifest JSON Source.
    pub const MANIFEST_JSON_SOURCE: Self = Self(MANIFEST_JSON_SOURCE);
    /// Manifest YAML Source.
    pub const MANIFEST_YAML_SOURCE: Self = Self(MANIFEST_YAML_SOURCE);

    /// Returns the raw numeric label.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.0
    }
}

impl From<TextValue> for i32 {
    fn from(value: TextValue) -> Self {
        value.0
    }
}

impl TryFrom<i32> for TextValue {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if is_known(value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

/// Returns `true` if `label` is a currently assigned SUIT Text Value label.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        MANIFEST_DESCRIPTION | UPDATE_DESCRIPTION | MANIFEST_JSON_SOURCE | MANIFEST_YAML_SOURCE
    )
}
