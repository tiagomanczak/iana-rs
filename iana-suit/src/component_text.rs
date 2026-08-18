//! SUIT Component Text Values (IANA registry: suit-component-text-values).
//!
//! Reference: [RFC-ietf-suit-manifest-34](https://www.iana.org/go/draft-ietf-suit-manifest-34)


/// Vendor Name.
const VENDOR_NAME: i32 = 1;
/// Model Name.
const MODEL_NAME: i32 = 2;
/// Vendor Domain.
const VENDOR_DOMAIN: i32 = 3;
/// Model Info.
const MODEL_INFO: i32 = 4;
/// Component Description.
const COMPONENT_DESCRIPTION: i32 = 5;
/// Component Version.
const COMPONENT_VERSION: i32 = 6;

/// A SUIT Component Text Value label.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ComponentTextValue(i32);

impl ComponentTextValue {
    /// Vendor Name.
    pub const VENDOR_NAME: Self = Self(VENDOR_NAME);
    /// Model Name.
    pub const MODEL_NAME: Self = Self(MODEL_NAME);
    /// Vendor Domain.
    pub const VENDOR_DOMAIN: Self = Self(VENDOR_DOMAIN);
    /// Model Info.
    pub const MODEL_INFO: Self = Self(MODEL_INFO);
    /// Component Description.
    pub const COMPONENT_DESCRIPTION: Self = Self(COMPONENT_DESCRIPTION);
    /// Component Version.
    pub const COMPONENT_VERSION: Self = Self(COMPONENT_VERSION);

    /// Returns the raw numeric label.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.0
    }
}

impl From<ComponentTextValue> for i32 {
    fn from(value: ComponentTextValue) -> Self {
        value.0
    }
}

impl TryFrom<i32> for ComponentTextValue {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if is_known(value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

/// Returns `true` if `label` is a currently assigned SUIT Component Text Value label.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        VENDOR_NAME
            | MODEL_NAME
            | VENDOR_DOMAIN
            | MODEL_INFO
            | COMPONENT_DESCRIPTION
            | COMPONENT_VERSION
    )
}
