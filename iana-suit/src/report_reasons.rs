//! SUIT Report Reasons (IANA registry: suit-report-reasons).
//!
//! Reference: RFC-ietf-suit-report-19

/// SUIT Report reason labels.
///
/// | Label | Name                              |
/// |-------|-----------------------------------|
/// | 0     | Result OK                         |
/// | 1     | CBOR Parse Failure                |
/// | 2     | Unsupported COSE Structure or Header |
/// | 3     | Unsupported COSE Algorithm        |
/// | 4     | Signature / MAC verification failed |
/// | 5     | Unsupported SUIT Command          |
/// | 6     | Unsupported SUIT Component        |
/// | 7     | Unauthorized SUIT Component       |
/// | 8     | Unsupported SUIT Parameter        |
/// | 9     | Severing Unsupported              |
/// | 10    | Condition Failed                  |
/// | 11    | Operation Failed                  |
/// | 12    | Invocation Pending                |
/// Result OK.
pub const RESULT_OK: i32 = 0;
/// CBOR Parse Failure.
pub const CBOR_PARSE_FAILURE: i32 = 1;
/// Unsupported COSE Structure or Header.
pub const UNSUPPORTED_COSE_STRUCTURE_OR_HEADER: i32 = 2;
/// Unsupported COSE Algorithm.
pub const UNSUPPORTED_COSE_ALGORITHM: i32 = 3;
/// Signature / MAC verification failed.
pub const SIGNATURE_MAC_VERIFICATION_FAILED: i32 = 4;
/// Unsupported SUIT Command.
pub const UNSUPPORTED_SUIT_COMMAND: i32 = 5;
/// Unsupported SUIT Component.
pub const UNSUPPORTED_SUIT_COMPONENT: i32 = 6;
/// Unauthorized SUIT Component.
pub const UNAUTHORIZED_SUIT_COMPONENT: i32 = 7;
/// Unsupported SUIT Parameter.
pub const UNSUPPORTED_SUIT_PARAMETER: i32 = 8;
/// Severing Unsupported.
pub const SEVERING_UNSUPPORTED: i32 = 9;
/// Condition Failed.
pub const CONDITION_FAILED: i32 = 10;
/// Operation Failed.
pub const OPERATION_FAILED: i32 = 11;
/// Invocation Pending.
pub const INVOCATION_PENDING: i32 = 12;

/// Returns `true` if `label` is a currently assigned SUIT Report Reason label.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        RESULT_OK
            | CBOR_PARSE_FAILURE
            | UNSUPPORTED_COSE_STRUCTURE_OR_HEADER
            | UNSUPPORTED_COSE_ALGORITHM
            | SIGNATURE_MAC_VERIFICATION_FAILED
            | UNSUPPORTED_SUIT_COMMAND
            | UNSUPPORTED_SUIT_COMPONENT
            | UNAUTHORIZED_SUIT_COMPONENT
            | UNSUPPORTED_SUIT_PARAMETER
            | SEVERING_UNSUPPORTED
            | CONDITION_FAILED
            | OPERATION_FAILED
            | INVOCATION_PENDING
    )
}
