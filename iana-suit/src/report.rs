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
pub mod label {
    /// Nonce.
    pub const NONCE: i64 = 2;
    /// Records.
    pub const RECORDS: i64 = 3;
    /// Result.
    pub const RESULT: i64 = 4;
    /// Result Code.
    pub const RESULT_CODE: i64 = 5;
    /// Result Record.
    pub const RESULT_RECORD: i64 = 6;
    /// Result Reason.
    pub const RESULT_REASON: i64 = 7;
    /// Capability Report.
    pub const CAPABILITY_REPORT: i64 = 8;
    /// Reference.
    pub const REFERENCE: i64 = 99;
}
