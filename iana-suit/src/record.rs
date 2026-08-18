//! SUIT Record Elements (IANA registry: suit-record-elements).
//!
//! Reference: [RFC-ietf-suit-report-19](https://www.iana.org/go/draft-ietf-suit-report-19)

const MANIFEST_ID: i32 = 0;
/// Manifest Section.
const MANIFEST_SECTION: i32 = 1;
/// Section Offset.
const SECTION_OFFSET: i32 = 2;
/// Component Index.
const COMPONENT_INDEX: i32 = 3;
/// Record Properties.
const RECORD_PROPERTIES: i32 = 4;

/// A SUIT Record Element label.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RecordElement(i32);

impl RecordElement {
    /// Manifest ID.
    pub const MANIFEST_ID: Self = Self(MANIFEST_ID);
    /// Manifest Section.
    pub const MANIFEST_SECTION: Self = Self(MANIFEST_SECTION);
    /// Section Offset.
    pub const SECTION_OFFSET: Self = Self(SECTION_OFFSET);
    /// Component Index.
    pub const COMPONENT_INDEX: Self = Self(COMPONENT_INDEX);
    /// Record Properties.
    pub const RECORD_PROPERTIES: Self = Self(RECORD_PROPERTIES);

    /// Returns the raw numeric label.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.0
    }
}

impl From<RecordElement> for i32 {
    fn from(value: RecordElement) -> Self {
        value.0
    }
}

impl TryFrom<i32> for RecordElement {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if is_known(value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

/// Returns `true` if `label` is a currently assigned SUIT Record Element label.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        MANIFEST_ID | MANIFEST_SECTION | SECTION_OFFSET | COMPONENT_INDEX | RECORD_PROPERTIES
    )
}
