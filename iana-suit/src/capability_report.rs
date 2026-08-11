//! SUIT Capability Report Elements (IANA registry: suit-capability-report-elements).
//!
//! Reference: RFC-ietf-suit-report-19

/// SUIT Capability Report element labels.
///
/// | Label | Name                      |
/// |-------|---------------------------|
/// | 1     | Components                |
/// | 2     | Commands                  |
/// | 3     | Parameters                |
/// | 4     | Cryptographic Algorithms  |
/// | 5     | Envelope Elements         |
/// | 6     | Manifest Elements         |
/// | 7     | Common Elements           |
/// | 8     | Text Elements             |
/// | 9     | Component Text Elements   |
/// | 10    | Dependency Capabilities  |
pub mod label {
    /// Components.
    pub const COMPONENTS: i64 = 1;
    /// Commands.
    pub const COMMANDS: i64 = 2;
    /// Parameters.
    pub const PARAMETERS: i64 = 3;
    /// Cryptographic Algorithms.
    pub const CRYPTOGRAPHIC_ALGORITHMS: i64 = 4;
    /// Envelope Elements.
    pub const ENVELOPE_ELEMENTS: i64 = 5;
    /// Manifest Elements.
    pub const MANIFEST_ELEMENTS: i64 = 6;
    /// Common Elements.
    pub const COMMON_ELEMENTS: i64 = 7;
    /// Text Elements.
    pub const TEXT_ELEMENTS: i64 = 8;
    /// Component Text Elements.
    pub const COMPONENT_TEXT_ELEMENTS: i64 = 9;
    /// Dependency Capabilities.
    pub const DEPENDENCY_CAPABILITIES: i64 = 10;
}
