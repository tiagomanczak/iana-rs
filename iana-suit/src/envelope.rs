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
/// Sentinel value indicating an unset field. **Not a valid CBOR map key for encoding.**
pub const UNSET_DETECTION: i32 = 0;
/// Reserved (Delegation).
pub const DELEGATION: i32 = 1;
/// Authentication Wrapper.
pub const AUTHENTICATION_WRAPPER: i32 = 2;
/// Manifest.
pub const MANIFEST: i32 = 3;
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

/// Returns `true` if `label` is a currently assigned SUIT Envelope Element label.
///
/// `UNSET_DETECTION` (value `0`) is intentionally excluded — it is a sentinel
/// value, not a valid CBOR map key for encoding.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        DELEGATION
            | AUTHENTICATION_WRAPPER
            | MANIFEST
            | DEPENDENCY_RESOLUTION
            | PAYLOAD_FETCH
            | CANDIDATE_VERIFICATION
            | PAYLOAD_INSTALLATION
            | TEXT_DESCRIPTION
    )
}

/// SUIT Envelope CBOR tag values.
///
/// | Tag | Name          |
/// |-----|---------------|
/// | 107 | SUIT Envelope |
pub mod tag {

    /// SUIT Envelope.
    pub const SUIT_ENVELOPE: i32 = 107;

    /// Returns `true` if `label` is a currently assigned SUIT Envelope tag.
    #[must_use]
    pub const fn is_known(label: i32) -> bool {
        matches!(label, SUIT_ENVELOPE)
    }
}
