//! SUIT Commands (IANA registry: suit-commands).
//!
//! Reference: RFC-ietf-suit-manifest-34

/// SUIT Command labels.
///
/// | Label | Name                  |
/// |-------|-----------------------|
/// | 0     | Unset Detection       |
/// | 1     | Vendor Identifier     |
/// | 2     | Class Identifier      |
/// | 3     | Image Match           |
/// | 5     | Component Slot        |
/// | 6     | Check Content         |
/// | 7     | Dependency Integrity  |
/// | 8     | Is Dependency         |
/// | 11    | Process Dependency    |
/// | 12    | Set Component Index   |
/// | 14    | Abort                 |
/// | 15    | Try Each              |
/// | 18    | Write Content         |
/// | 19    | Set Parameters        |
/// | 20    | Override Parameters   |
/// | 21    | Fetch                 |
/// | 22    | Copy                  |
/// | 23    | Invoke                |
/// | 24    | Device Identifier     |
/// | 31    | Swap                  |
/// | 32    | Run Sequence          |
/// | 33    | Unlink                |
pub mod label {
    /// Unset Detection.
    pub const UNSET_DETECTION: i64 = 0;
    /// Vendor Identifier.
    pub const VENDOR_IDENTIFIER: i64 = 1;
    /// Class Identifier.
    pub const CLASS_IDENTIFIER: i64 = 2;
    /// Image Match.
    pub const IMAGE_MATCH: i64 = 3;
    /// Component Slot.
    pub const COMPONENT_SLOT: i64 = 5;
    /// Check Content.
    pub const CHECK_CONTENT: i64 = 6;
    /// Dependency Integrity.
    pub const DEPENDENCY_INTEGRITY: i64 = 7;
    /// Is Dependency.
    pub const IS_DEPENDENCY: i64 = 8;
    /// Process Dependency.
    pub const PROCESS_DEPENDENCY: i64 = 11;
    /// Set Component Index.
    pub const SET_COMPONENT_INDEX: i64 = 12;
    /// Abort.
    pub const ABORT: i64 = 14;
    /// Try Each.
    pub const TRY_EACH: i64 = 15;
    /// Write Content.
    pub const WRITE_CONTENT: i64 = 18;
    /// Set Parameters.
    pub const SET_PARAMETERS: i64 = 19;
    /// Override Parameters.
    pub const OVERRIDE_PARAMETERS: i64 = 20;
    /// Fetch.
    pub const FETCH: i64 = 21;
    /// Copy.
    pub const COPY: i64 = 22;
    /// Invoke.
    pub const INVOKE: i64 = 23;
    /// Device Identifier.
    pub const DEVICE_IDENTIFIER: i64 = 24;
    /// Swap.
    pub const SWAP: i64 = 31;
    /// Run Sequence.
    pub const RUN_SEQUENCE: i64 = 32;
    /// Unlink.
    pub const UNLINK: i64 = 33;
}
