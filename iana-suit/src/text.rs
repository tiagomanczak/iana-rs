//! SUIT Text Values (IANA registry: suit-text-values).
//!
//! Reference: RFC-ietf-suit-manifest-34

/// SUIT Text value labels.
///
/// | Label | Name                 |
/// |-------|----------------------|
/// | 0     | Unset Detection      |
/// | 1     | Manifest Description |
/// | 2     | Update Description   |
/// | 3     | Manifest JSON Source |
/// | 4     | Manifest YAML Source |
/// Sentinel value indicating an unset field. **Not a valid CBOR map key for encoding.**
pub const UNSET_DETECTION: i32 = 0;
/// Manifest Description.
pub const MANIFEST_DESCRIPTION: i32 = 1;
/// Update Description.
pub const UPDATE_DESCRIPTION: i32 = 2;
/// Manifest JSON Source.
pub const MANIFEST_JSON_SOURCE: i32 = 3;
/// Manifest YAML Source.
pub const MANIFEST_YAML_SOURCE: i32 = 4;

/// Returns `true` if `label` is a currently assigned SUIT Text Value label.
///
/// `UNSET_DETECTION` (value `0`) is intentionally excluded — it is a sentinel
/// value, not a valid CBOR map key for encoding.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        MANIFEST_DESCRIPTION | UPDATE_DESCRIPTION | MANIFEST_JSON_SOURCE | MANIFEST_YAML_SOURCE
    )
}
