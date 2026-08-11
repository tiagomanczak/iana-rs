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
pub mod label {
    /// Result OK.
    pub const RESULT_OK: i64 = 0;
    /// CBOR Parse Failure.
    pub const CBOR_PARSE_FAILURE: i64 = 1;
    /// Unsupported COSE Structure or Header.
    pub const UNSUPPORTED_COSE_STRUCTURE_OR_HEADER: i64 = 2;
    /// Unsupported COSE Algorithm.
    pub const UNSUPPORTED_COSE_ALGORITHM: i64 = 3;
    /// Signature / MAC verification failed.
    pub const SIGNATURE_MAC_VERIFICATION_FAILED: i64 = 4;
    /// Unsupported SUIT Command.
    pub const UNSUPPORTED_SUIT_COMMAND: i64 = 5;
    /// Unsupported SUIT Component.
    pub const UNSUPPORTED_SUIT_COMPONENT: i64 = 6;
    /// Unauthorized SUIT Component.
    pub const UNAUTHORIZED_SUIT_COMPONENT: i64 = 7;
    /// Unsupported SUIT Parameter.
    pub const UNSUPPORTED_SUIT_PARAMETER: i64 = 8;
    /// Severing Unsupported.
    pub const SEVERING_UNSUPPORTED: i64 = 9;
    /// Condition Failed.
    pub const CONDITION_FAILED: i64 = 10;
    /// Operation Failed.
    pub const OPERATION_FAILED: i64 = 11;
    /// Invocation Pending.
    pub const INVOCATION_PENDING: i64 = 12;
}
