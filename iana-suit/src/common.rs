//! SUIT Common Elements (IANA registry: suit-common-elements).
//!
//! Reference: RFC-ietf-suit-manifest-34

/// SUIT Common element labels.
///
/// | Label | Name                   |
/// |-------|------------------------|
/// | 0     | Unset Detection        |
/// | 1     | Dependencies           |
/// | 2     | Component Identifiers  |
/// | 4     | Common Command Sequence|
/// Sentinel value indicating an unset field. **Not a valid CBOR map key for encoding.**
pub const UNSET_DETECTION: i32 = 0;
/// Dependencies.
pub const DEPENDENCIES: i32 = 1;
/// Component Identifiers.
pub const COMPONENT_IDENTIFIERS: i32 = 2;
/// Common Command Sequence.
pub const COMMON_COMMAND_SEQUENCE: i32 = 4;

/// Returns `true` if `label` is a currently assigned SUIT Common Element label.
///
/// `UNSET_DETECTION` (value `0`) is intentionally excluded — it is a sentinel
/// value, not a valid CBOR map key for encoding.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        DEPENDENCIES | COMPONENT_IDENTIFIERS | COMMON_COMMAND_SEQUENCE
    )
}
