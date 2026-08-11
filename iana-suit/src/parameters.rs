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
pub mod label {
    /// Unset Detection.
    pub const UNSET_DETECTION: i64 = 0;
    /// Vendor ID.
    pub const VENDOR_ID: i64 = 1;
    /// Class ID.
    pub const CLASS_ID: i64 = 2;
    /// Image Digest.
    pub const IMAGE_DIGEST: i64 = 3;
    /// Component Slot.
    pub const COMPONENT_SLOT: i64 = 5;
    /// Strict Order.
    pub const STRICT_ORDER: i64 = 12;
    /// Soft Failure.
    pub const SOFT_FAILURE: i64 = 13;
    /// Image Size.
    pub const IMAGE_SIZE: i64 = 14;
    /// Content.
    pub const CONTENT: i64 = 18;
    /// Encryption Info.
    pub const ENCRYPTION_INFO: i64 = 19;
    /// URI.
    pub const URI: i64 = 21;
    /// Source Component.
    pub const SOURCE_COMPONENT: i64 = 22;
    /// Invoke Args.
    pub const INVOKE_ARGS: i64 = 23;
    /// Device ID.
    pub const DEVICE_ID: i64 = 24;
}
