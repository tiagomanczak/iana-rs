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
/// Components.
pub const COMPONENTS: i32 = 1;
/// Commands.
pub const COMMANDS: i32 = 2;
/// Parameters.
pub const PARAMETERS: i32 = 3;
/// Cryptographic Algorithms.
pub const CRYPTOGRAPHIC_ALGORITHMS: i32 = 4;
/// Envelope Elements.
pub const ENVELOPE_ELEMENTS: i32 = 5;
/// Manifest Elements.
pub const MANIFEST_ELEMENTS: i32 = 6;
/// Common Elements.
pub const COMMON_ELEMENTS: i32 = 7;
/// Text Elements.
pub const TEXT_ELEMENTS: i32 = 8;
/// Component Text Elements.
pub const COMPONENT_TEXT_ELEMENTS: i32 = 9;
/// Dependency Capabilities.
pub const DEPENDENCY_CAPABILITIES: i32 = 10;

/// Returns `true` if `label` is a currently assigned SUIT Capability Report Element label.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        COMPONENTS
            | COMMANDS
            | PARAMETERS
            | CRYPTOGRAPHIC_ALGORITHMS
            | ENVELOPE_ELEMENTS
            | MANIFEST_ELEMENTS
            | COMMON_ELEMENTS
            | TEXT_ELEMENTS
            | COMPONENT_TEXT_ELEMENTS
            | DEPENDENCY_CAPABILITIES
    )
}
