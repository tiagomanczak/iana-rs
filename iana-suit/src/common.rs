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
pub mod label {
    /// Unset Detection.
    pub const UNSET_DETECTION: i64 = 0;
    /// Dependencies.
    pub const DEPENDENCIES: i64 = 1;
    /// Component Identifiers.
    pub const COMPONENT_IDENTIFIERS: i64 = 2;
    /// Common Command Sequence.
    pub const COMMON_COMMAND_SEQUENCE: i64 = 4;
}
