//! SUIT Report Elements (IANA registry: suit-report-elements).
//!
//! Reference: RFC-ietf-suit-report-19

/// SUIT Report element labels.
///
/// | Label | Name              |
/// |-------|-------------------|
/// | 2     | Nonce             |
/// | 3     | Records           |
/// | 4     | Result            |
/// | 5     | Result Code       |
/// | 6     | Result Record     |
/// | 7     | Result Reason     |
/// | 8     | Capability Report |
/// | 99    | Reference         |
/// Nonce.
pub const NONCE: i32 = 2;
/// Records.
pub const RECORDS: i32 = 3;
/// Result.
pub const RESULT: i32 = 4;
/// Result Code.
pub const RESULT_CODE: i32 = 5;
/// Result Record.
pub const RESULT_RECORD: i32 = 6;
/// Result Reason.
pub const RESULT_REASON: i32 = 7;
/// Capability Report.
pub const CAPABILITY_REPORT: i32 = 8;
/// Reference.
pub const REFERENCE: i32 = 99;

/// Returns `true` if `label` is a currently assigned SUIT Report Element label.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        NONCE
            | RECORDS
            | RESULT
            | RESULT_CODE
            | RESULT_RECORD
            | RESULT_REASON
            | CAPABILITY_REPORT
            | REFERENCE
    )
}
