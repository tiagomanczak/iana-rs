//! SUIT Component Text Values (IANA registry: suit-component-text-values).
//!
//! Reference: RFC-ietf-suit-manifest-34

/// SUIT Component Text value labels.
///
/// | Label | Name                 |
/// |-------|----------------------|
/// | 0     | Unset Detection      |
/// | 1     | Vendor Name          |
/// | 2     | Model Name           |
/// | 3     | Vendor Domain        |
/// | 4     | Model Info           |
/// | 5     | Component Description |
/// | 6     | Component Version    |
pub mod label {
    /// Unset Detection.
    pub const UNSET_DETECTION: i64 = 0;
    /// Vendor Name.
    pub const VENDOR_NAME: i64 = 1;
    /// Model Name.
    pub const MODEL_NAME: i64 = 2;
    /// Vendor Domain.
    pub const VENDOR_DOMAIN: i64 = 3;
    /// Model Info.
    pub const MODEL_INFO: i64 = 4;
    /// Component Description.
    pub const COMPONENT_DESCRIPTION: i64 = 5;
    /// Component Version.
    pub const COMPONENT_VERSION: i64 = 6;
}
