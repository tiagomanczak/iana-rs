//! SUIT Manifest Elements (IANA registry: suit-manifest-elements).
//!
//! Reference: [RFC-ietf-suit-manifest-34](https://www.iana.org/go/draft-ietf-suit-manifest-34)

/// Encoding Version.
const ENCODING_VERSION: i32 = 1;
/// Sequence Number.
const SEQUENCE_NUMBER: i32 = 2;
/// Common Data.
const COMMON_DATA: i32 = 3;
/// Reference URI.
const REFERENCE_URI: i32 = 4;
/// Manifest Component ID.
const MANIFEST_COMPONENT_ID: i32 = 5;
/// Image Validation.
const IMAGE_VALIDATION: i32 = 7;
/// Image Loading.
const IMAGE_LOADING: i32 = 8;
/// Image Invocation.
const IMAGE_INVOCATION: i32 = 9;
/// Dependency Resolution.
const DEPENDENCY_RESOLUTION: i32 = 15;
/// Payload Fetch.
const PAYLOAD_FETCH: i32 = 16;
/// Candidate Verification.
const CANDIDATE_VERIFICATION: i32 = 18;
/// Payload Installation.
const PAYLOAD_INSTALLATION: i32 = 20;
/// Text Description.
const TEXT_DESCRIPTION: i32 = 23;
/// Uninstall.
const UNINSTALL: i32 = 24;
/// Manufacturer Usage Description (MUD).
const MANUFACTURER_USAGE_DESCRIPTION: i32 = 25;

/// A SUIT Manifest Element label.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ManifestElement(i32);

impl ManifestElement {
    /// Encoding Version.
    pub const ENCODING_VERSION: Self = Self(ENCODING_VERSION);
    /// Sequence Number.
    pub const SEQUENCE_NUMBER: Self = Self(SEQUENCE_NUMBER);
    /// Common Data.
    pub const COMMON_DATA: Self = Self(COMMON_DATA);
    /// Reference URI.
    pub const REFERENCE_URI: Self = Self(REFERENCE_URI);
    /// Manifest Component ID.
    pub const MANIFEST_COMPONENT_ID: Self = Self(MANIFEST_COMPONENT_ID);
    /// Image Validation.
    pub const IMAGE_VALIDATION: Self = Self(IMAGE_VALIDATION);
    /// Image Loading.
    pub const IMAGE_LOADING: Self = Self(IMAGE_LOADING);
    /// Image Invocation.
    pub const IMAGE_INVOCATION: Self = Self(IMAGE_INVOCATION);
    /// Dependency Resolution.
    pub const DEPENDENCY_RESOLUTION: Self = Self(DEPENDENCY_RESOLUTION);
    /// Payload Fetch.
    pub const PAYLOAD_FETCH: Self = Self(PAYLOAD_FETCH);
    /// Candidate Verification.
    pub const CANDIDATE_VERIFICATION: Self = Self(CANDIDATE_VERIFICATION);
    /// Payload Installation.
    pub const PAYLOAD_INSTALLATION: Self = Self(PAYLOAD_INSTALLATION);
    /// Text Description.
    pub const TEXT_DESCRIPTION: Self = Self(TEXT_DESCRIPTION);
    /// Uninstall.
    pub const UNINSTALL: Self = Self(UNINSTALL);
    /// Manufacturer Usage Description (MUD).
    pub const MANUFACTURER_USAGE_DESCRIPTION: Self = Self(MANUFACTURER_USAGE_DESCRIPTION);

    /// Returns the raw numeric label.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.0
    }
}

impl From<ManifestElement> for i32 {
    fn from(value: ManifestElement) -> Self {
        value.0
    }
}

impl TryFrom<i32> for ManifestElement {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if is_known(value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

/// Returns `true` if `label` is a currently assigned SUIT Manifest Element label.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        ENCODING_VERSION
            | SEQUENCE_NUMBER
            | COMMON_DATA
            | REFERENCE_URI
            | MANIFEST_COMPONENT_ID
            | IMAGE_VALIDATION
            | IMAGE_LOADING
            | IMAGE_INVOCATION
            | DEPENDENCY_RESOLUTION
            | PAYLOAD_FETCH
            | CANDIDATE_VERIFICATION
            | PAYLOAD_INSTALLATION
            | TEXT_DESCRIPTION
            | UNINSTALL
            | MANUFACTURER_USAGE_DESCRIPTION
    )
}
