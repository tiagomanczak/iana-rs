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
pub mod label {
    /// Unset Detection.
    pub const UNSET_DETECTION: i64 = 0;
    /// Encoding Version.
    pub const ENCODING_VERSION: i64 = 1;
    /// Sequence Number.
    pub const SEQUENCE_NUMBER: i64 = 2;
    /// Common Data.
    pub const COMMON_DATA: i64 = 3;
    /// Reference URI.
    pub const REFERENCE_URI: i64 = 4;
    /// Manifest Component ID.
    pub const MANIFEST_COMPONENT_ID: i64 = 5;
    /// Image Validation.
    pub const IMAGE_VALIDATION: i64 = 7;
    /// Image Loading.
    pub const IMAGE_LOADING: i64 = 8;
    /// Image Invocation.
    pub const IMAGE_INVOCATION: i64 = 9;
    /// Dependency Resolution.
    pub const DEPENDENCY_RESOLUTION: i64 = 15;
    /// Payload Fetch.
    pub const PAYLOAD_FETCH: i64 = 16;
    /// Candidate Verification.
    pub const CANDIDATE_VERIFICATION: i64 = 18;
    /// Payload Installation.
    pub const PAYLOAD_INSTALLATION: i64 = 20;
    /// Text Description.
    pub const TEXT_DESCRIPTION: i64 = 23;
    /// Uninstall.
    pub const UNINSTALL: i64 = 24;
    /// Manufacturer Usage Description (MUD).
    pub const MANUFACTURER_USAGE_DESCRIPTION: i64 = 25;
}
