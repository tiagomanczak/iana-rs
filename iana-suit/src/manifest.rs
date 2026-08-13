//! SUIT Manifest Elements (IANA registry: suit-manifest-elements).
//!
//! Reference: RFC-ietf-suit-manifest-34

/// SUIT Manifest element labels.
///
/// | Label | Name                        |
/// |-------|-----------------------------|
/// | 0     | Unset Detection             |
/// | 1     | Encoding Version            |
/// | 2     | Sequence Number             |
/// | 3     | Common Data                 |
/// | 4     | Reference URI               |
/// | 5     | Manifest Component ID       |
/// | 7     | Image Validation            |
/// | 8     | Image Loading               |
/// | 9     | Image Invocation            |
/// | 15    | Dependency Resolution       |
/// | 16    | Payload Fetch               |
/// | 18    | Candidate Verification      |
/// | 20    | Payload Installation        |
/// | 23    | Text Description            |
/// | 24    | Uninstall                   |
/// | 25    | Manufacturer Usage Desc.    |
/// Sentinel value indicating an unset field. **Not a valid CBOR map key for encoding.**
pub const UNSET_DETECTION: i32 = 0;
/// Encoding Version.
pub const ENCODING_VERSION: i32 = 1;
/// Sequence Number.
pub const SEQUENCE_NUMBER: i32 = 2;
/// Common Data.
pub const COMMON_DATA: i32 = 3;
/// Reference URI.
pub const REFERENCE_URI: i32 = 4;
/// Manifest Component ID.
pub const MANIFEST_COMPONENT_ID: i32 = 5;
/// Image Validation.
pub const IMAGE_VALIDATION: i32 = 7;
/// Image Loading.
pub const IMAGE_LOADING: i32 = 8;
/// Image Invocation.
pub const IMAGE_INVOCATION: i32 = 9;
/// Dependency Resolution.
pub const DEPENDENCY_RESOLUTION: i32 = 15;
/// Payload Fetch.
pub const PAYLOAD_FETCH: i32 = 16;
/// Candidate Verification.
pub const CANDIDATE_VERIFICATION: i32 = 18;
/// Payload Installation.
pub const PAYLOAD_INSTALLATION: i32 = 20;
/// Text Description.
pub const TEXT_DESCRIPTION: i32 = 23;
/// Uninstall.
pub const UNINSTALL: i32 = 24;
/// Manufacturer Usage Description (MUD).
pub const MANUFACTURER_USAGE_DESCRIPTION: i32 = 25;

/// Returns `true` if `label` is a currently assigned SUIT Manifest Element label.
///
/// `UNSET_DETECTION` (value `0`) is intentionally excluded — it is a sentinel
/// value, not a valid CBOR map key for encoding.
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
