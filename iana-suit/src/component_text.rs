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
/// Sentinel value indicating an unset field. **Not a valid CBOR map key for encoding.**
pub const UNSET_DETECTION: i32 = 0;
/// Vendor Name.
pub const VENDOR_NAME: i32 = 1;
/// Model Name.
pub const MODEL_NAME: i32 = 2;
/// Vendor Domain.
pub const VENDOR_DOMAIN: i32 = 3;
/// Model Info.
pub const MODEL_INFO: i32 = 4;
/// Component Description.
pub const COMPONENT_DESCRIPTION: i32 = 5;
/// Component Version.
pub const COMPONENT_VERSION: i32 = 6;

/// Returns `true` if `label` is a currently assigned SUIT Component Text Value label.
///
/// `UNSET_DETECTION` (value `0`) is intentionally excluded — it is a sentinel
/// value, not a valid CBOR map key for encoding.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        VENDOR_NAME
            | MODEL_NAME
            | VENDOR_DOMAIN
            | MODEL_INFO
            | COMPONENT_DESCRIPTION
            | COMPONENT_VERSION
    )
}
