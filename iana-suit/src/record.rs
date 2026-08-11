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
pub mod label {
    /// Manifest ID.
    pub const MANIFEST_ID: i64 = 0;
    /// Manifest Section.
    pub const MANIFEST_SECTION: i64 = 1;
    /// Section Offset.
    pub const SECTION_OFFSET: i64 = 2;
    /// Component Index.
    pub const COMPONENT_INDEX: i64 = 3;
    /// Record Properties.
    pub const RECORD_PROPERTIES: i64 = 4;
}
