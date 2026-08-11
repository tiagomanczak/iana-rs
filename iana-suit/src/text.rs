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
pub mod label {
    /// Unset Detection.
    pub const UNSET_DETECTION: i64 = 0;
    /// Manifest Description.
    pub const MANIFEST_DESCRIPTION: i64 = 1;
    /// Update Description.
    pub const UPDATE_DESCRIPTION: i64 = 2;
    /// Manifest JSON Source.
    pub const MANIFEST_JSON_SOURCE: i64 = 3;
    /// Manifest YAML Source.
    pub const MANIFEST_YAML_SOURCE: i64 = 4;
}
