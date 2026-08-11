//! SUIT Envelope Elements (IANA registry: suit-envelope-elements).
//!
//! Reference: RFC-ietf-suit-manifest-34

/// SUIT Envelope element labels.
///
/// | Label | Name                   |
/// |-------|------------------------|
/// | 0     | Unset Detection        |
/// | 1     | Reserved (Delegation)  |
/// | 2     | Authentication Wrapper |
/// | 3     | Manifest               |
/// | 15    | Dependency Resolution  |
/// | 16    | Payload Fetch          |
/// | 18    | Candidate Verification |
/// | 20    | Payload Installation   |
/// | 23    | Text Description       |
pub mod label {
    /// Unset Detection.
    pub const UNSET_DETECTION: i64 = 0;
    /// Reserved (Delegation).
    pub const DELEGATION: i64 = 1;
    /// Authentication Wrapper.
    pub const AUTHENTICATION_WRAPPER: i64 = 2;
    /// Manifest.
    pub const MANIFEST: i64 = 3;
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
}
