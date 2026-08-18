//! SUIT Capability Report Elements (IANA registry: suit-capability-report-elements).
//!
//! Reference: [RFC-ietf-suit-report-19](https://www.iana.org/go/draft-ietf-suit-report-19)


/// Components.
const COMPONENTS: i32 = 1;
/// Commands.
const COMMANDS: i32 = 2;
/// Parameters.
const PARAMETERS: i32 = 3;
/// Cryptographic Algorithms.
const CRYPTOGRAPHIC_ALGORITHMS: i32 = 4;
/// Envelope Elements.
const ENVELOPE_ELEMENTS: i32 = 5;
/// Manifest Elements.
const MANIFEST_ELEMENTS: i32 = 6;
/// Common Elements.
const COMMON_ELEMENTS: i32 = 7;
/// Text Elements.
const TEXT_ELEMENTS: i32 = 8;
/// Component Text Elements.
const COMPONENT_TEXT_ELEMENTS: i32 = 9;
/// Dependency Capabilities.
const DEPENDENCY_CAPABILITIES: i32 = 10;


/// A SUIT Capability Report Element label.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CapabilityReportElement(i32);

impl CapabilityReportElement {
    /// Components.
    pub const COMPONENTS: Self = Self(COMPONENTS);
    /// Commands.
    pub const COMMANDS: Self = Self(COMMANDS);
    /// Parameters.
    pub const PARAMETERS: Self = Self(PARAMETERS);
    /// Cryptographic Algorithms.
    pub const CRYPTOGRAPHIC_ALGORITHMS: Self = Self(CRYPTOGRAPHIC_ALGORITHMS);
    /// Envelope Elements.
    pub const ENVELOPE_ELEMENTS: Self = Self(ENVELOPE_ELEMENTS);
    /// Manifest Elements.
    pub const MANIFEST_ELEMENTS: Self = Self(MANIFEST_ELEMENTS);
    /// Common Elements.
    pub const COMMON_ELEMENTS: Self = Self(COMMON_ELEMENTS);
    /// Text Elements.
    pub const TEXT_ELEMENTS: Self = Self(TEXT_ELEMENTS);
    /// Component Text Elements.
    pub const COMPONENT_TEXT_ELEMENTS: Self = Self(COMPONENT_TEXT_ELEMENTS);
    /// Dependency Capabilities.
    pub const DEPENDENCY_CAPABILITIES: Self = Self(DEPENDENCY_CAPABILITIES);

    /// Returns the raw numeric label.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.0
    }
}

impl From<CapabilityReportElement> for i32 {
    fn from(value: CapabilityReportElement) -> Self {
        value.0
    }
}

impl TryFrom<i32> for CapabilityReportElement {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if is_known(value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

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
