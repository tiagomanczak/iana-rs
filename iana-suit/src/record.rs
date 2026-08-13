//! SUIT Record Elements (IANA registry: suit-record-elements).
//!
//! Reference: RFC-ietf-suit-report-19

/// SUIT Record element labels.
///
/// | Label | Name              |
/// |-------|-------------------|
/// | 0     | Manifest ID       |
/// | 1     | Manifest Section  |
/// | 2     | Section Offset    |
/// | 3     | Component Index   |
/// | 4     | Record Properties |
/// Manifest ID.
pub const MANIFEST_ID: i32 = 0;
/// Manifest Section.
pub const MANIFEST_SECTION: i32 = 1;
/// Section Offset.
pub const SECTION_OFFSET: i32 = 2;
/// Component Index.
pub const COMPONENT_INDEX: i32 = 3;
/// Record Properties.
pub const RECORD_PROPERTIES: i32 = 4;

/// Returns `true` if `label` is a currently assigned SUIT Record Element label.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        MANIFEST_ID | MANIFEST_SECTION | SECTION_OFFSET | COMPONENT_INDEX | RECORD_PROPERTIES
    )
}
