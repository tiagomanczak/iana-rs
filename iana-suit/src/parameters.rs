//! SUIT Parameters (IANA registry: suit-parameters).
//!
//! Reference: RFC-ietf-suit-manifest-34

/// SUIT Parameter labels.
///
/// | Label | Name              |
/// |-------|-------------------|
/// | 0     | Unset Detection   |
/// | 1     | Vendor ID         |
/// | 2     | Class ID          |
/// | 3     | Image Digest      |
/// | 5     | Component Slot    |
/// | 12    | Strict Order      |
/// | 13    | Soft Failure      |
/// | 14    | Image Size        |
/// | 18    | Content           |
/// | 19    | Encryption Info   |
/// | 21    | URI               |
/// | 22    | Source Component  |
/// | 23    | Invoke Args       |
/// | 24    | Device ID         |
/// Sentinel value indicating an unset field. **Not a valid CBOR map key for encoding.**
pub const UNSET_DETECTION: i32 = 0;
/// Vendor ID.
pub const VENDOR_ID: i32 = 1;
/// Class ID.
pub const CLASS_ID: i32 = 2;
/// Image Digest.
pub const IMAGE_DIGEST: i32 = 3;
/// Component Slot.
pub const COMPONENT_SLOT: i32 = 5;
/// Strict Order.
pub const STRICT_ORDER: i32 = 12;
/// Soft Failure.
pub const SOFT_FAILURE: i32 = 13;
/// Image Size.
pub const IMAGE_SIZE: i32 = 14;
/// Content.
pub const CONTENT: i32 = 18;
/// Encryption Info.
pub const ENCRYPTION_INFO: i32 = 19;
/// URI.
pub const URI: i32 = 21;
/// Source Component.
pub const SOURCE_COMPONENT: i32 = 22;
/// Invoke Args.
pub const INVOKE_ARGS: i32 = 23;
/// Device ID.
pub const DEVICE_ID: i32 = 24;

/// Returns `true` if `label` is a currently assigned SUIT Parameter label.
///
/// `UNSET_DETECTION` (value `0`) is intentionally excluded — it is a sentinel
/// value, not a valid CBOR map key for encoding.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        VENDOR_ID
            | CLASS_ID
            | IMAGE_DIGEST
            | COMPONENT_SLOT
            | STRICT_ORDER
            | SOFT_FAILURE
            | IMAGE_SIZE
            | CONTENT
            | ENCRYPTION_INFO
            | URI
            | SOURCE_COMPONENT
            | INVOKE_ARGS
            | DEVICE_ID
    )
}
