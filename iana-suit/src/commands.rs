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
/// Sentinel value indicating an unset field. **Not a valid CBOR map key for encoding.**
pub const UNSET_DETECTION: i32 = 0;
/// Vendor Identifier.
pub const VENDOR_IDENTIFIER: i32 = 1;
/// Class Identifier.
pub const CLASS_IDENTIFIER: i32 = 2;
/// Image Match.
pub const IMAGE_MATCH: i32 = 3;
/// Component Slot.
pub const COMPONENT_SLOT: i32 = 5;
/// Check Content.
pub const CHECK_CONTENT: i32 = 6;
/// Dependency Integrity.
pub const DEPENDENCY_INTEGRITY: i32 = 7;
/// Is Dependency.
pub const IS_DEPENDENCY: i32 = 8;
/// Process Dependency.
pub const PROCESS_DEPENDENCY: i32 = 11;
/// Set Component Index.
pub const SET_COMPONENT_INDEX: i32 = 12;
/// Abort.
pub const ABORT: i32 = 14;
/// Try Each.
pub const TRY_EACH: i32 = 15;
/// Write Content.
pub const WRITE_CONTENT: i32 = 18;
/// Set Parameters.
pub const SET_PARAMETERS: i32 = 19;
/// Override Parameters.
pub const OVERRIDE_PARAMETERS: i32 = 20;
/// Fetch.
pub const FETCH: i32 = 21;
/// Copy.
pub const COPY: i32 = 22;
/// Invoke.
pub const INVOKE: i32 = 23;
/// Device Identifier.
pub const DEVICE_IDENTIFIER: i32 = 24;
/// Swap.
pub const SWAP: i32 = 31;
/// Run Sequence.
pub const RUN_SEQUENCE: i32 = 32;
/// Unlink.
pub const UNLINK: i32 = 33;

/// Returns `true` if `label` is a currently assigned SUIT Command label.
///
/// `UNSET_DETECTION` (value `0`) is intentionally excluded — it is a sentinel
/// value, not a valid CBOR map key for encoding.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        VENDOR_IDENTIFIER
            | CLASS_IDENTIFIER
            | IMAGE_MATCH
            | COMPONENT_SLOT
            | CHECK_CONTENT
            | DEPENDENCY_INTEGRITY
            | IS_DEPENDENCY
            | PROCESS_DEPENDENCY
            | SET_COMPONENT_INDEX
            | ABORT
            | TRY_EACH
            | WRITE_CONTENT
            | SET_PARAMETERS
            | OVERRIDE_PARAMETERS
            | FETCH
            | COPY
            | INVOKE
            | DEVICE_IDENTIFIER
            | SWAP
            | RUN_SEQUENCE
            | UNLINK
    )
}
