//! SUIT Common Elements (IANA registry: suit-common-elements).
//!
//! Reference: [RFC-ietf-suit-manifest-34](https://www.iana.org/go/draft-ietf-suit-manifest-34)


/// Dependencies.
const DEPENDENCIES: i32 = 1;
/// Component Identifiers.
const COMPONENT_IDENTIFIERS: i32 = 2;
/// Common Command Sequence.
const COMMON_COMMAND_SEQUENCE: i32 = 4;

/// A SUIT Common Element label.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CommonElement(i32);

impl CommonElement {
    /// Dependencies.
    pub const DEPENDENCIES: Self = Self(DEPENDENCIES);
    /// Component Identifiers.
    pub const COMPONENT_IDENTIFIERS: Self = Self(COMPONENT_IDENTIFIERS);
    /// Common Command Sequence.
    pub const COMMON_COMMAND_SEQUENCE: Self = Self(COMMON_COMMAND_SEQUENCE);

    /// Returns the raw numeric label.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.0
    }
}

impl From<CommonElement> for i32 {
    fn from(value: CommonElement) -> Self {
        value.0
    }
}

impl TryFrom<i32> for CommonElement {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if is_known(value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

/// Returns `true` if `label` is a currently assigned SUIT Common Element label.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        DEPENDENCIES | COMPONENT_IDENTIFIERS | COMMON_COMMAND_SEQUENCE
    )
}
