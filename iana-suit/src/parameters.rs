//! SUIT Parameters (IANA registry: suit-parameters).
//!
//! Reference: [RFC-ietf-suit-manifest-34](https://www.iana.org/go/draft-ietf-suit-manifest-34)


/// Vendor ID.
const VENDOR_ID: i32 = 1;
/// Class ID.
const CLASS_ID: i32 = 2;
/// Image Digest.
const IMAGE_DIGEST: i32 = 3;
/// Component Slot.
const COMPONENT_SLOT: i32 = 5;
/// Strict Order.
const STRICT_ORDER: i32 = 12;
/// Soft Failure.
const SOFT_FAILURE: i32 = 13;
/// Image Size.
const IMAGE_SIZE: i32 = 14;
/// Content.
const CONTENT: i32 = 18;
/// Encryption Info.
const ENCRYPTION_INFO: i32 = 19;
/// URI.
const URI: i32 = 21;
/// Source Component.
const SOURCE_COMPONENT: i32 = 22;
/// Invoke Args.
const INVOKE_ARGS: i32 = 23;
/// Device ID.
const DEVICE_ID: i32 = 24;

/// A SUIT Parameter label.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Parameter(i32);

impl Parameter {
    /// Vendor ID.
    pub const VENDOR_ID: Self = Self(VENDOR_ID);
    /// Class ID.
    pub const CLASS_ID: Self = Self(CLASS_ID);
    /// Image Digest.
    pub const IMAGE_DIGEST: Self = Self(IMAGE_DIGEST);
    /// Component Slot.
    pub const COMPONENT_SLOT: Self = Self(COMPONENT_SLOT);
    /// Strict Order.
    pub const STRICT_ORDER: Self = Self(STRICT_ORDER);
    /// Soft Failure.
    pub const SOFT_FAILURE: Self = Self(SOFT_FAILURE);
    /// Image Size.
    pub const IMAGE_SIZE: Self = Self(IMAGE_SIZE);
    /// Content.
    pub const CONTENT: Self = Self(CONTENT);
    /// Encryption Info.
    pub const ENCRYPTION_INFO: Self = Self(ENCRYPTION_INFO);
    /// URI.
    pub const URI: Self = Self(URI);
    /// Source Component.
    pub const SOURCE_COMPONENT: Self = Self(SOURCE_COMPONENT);
    /// Invoke Args.
    pub const INVOKE_ARGS: Self = Self(INVOKE_ARGS);
    /// Device ID.
    pub const DEVICE_ID: Self = Self(DEVICE_ID);

    /// Returns the raw numeric label.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.0
    }
}

impl From<Parameter> for i32 {
    fn from(value: Parameter) -> Self {
        value.0
    }
}

impl TryFrom<i32> for Parameter {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if is_known(value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

/// Returns `true` if `label` is a currently assigned SUIT Parameter label.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        VENDOR_ID
            | CLASS_ID
            | IMAGE_DIGEST
            | COMPONENT_SLOT
            | STRICT_ORDER
            | SOFT_FAILURE
            | IMAGE_SIZE
            | CONTENT
            | ENCRYPTION_INFO
            | URI
            | SOURCE_COMPONENT
            | INVOKE_ARGS
            | DEVICE_ID
    )
}
