//! SUIT Report Reasons (IANA registry: suit-report-reasons).
//!
//! Reference: [RFC-ietf-suit-report-19](https://www.iana.org/go/draft-ietf-suit-report-19)

const RESULT_OK: i32 = 0;
/// CBOR Parse Failure.
const CBOR_PARSE_FAILURE: i32 = 1;
/// Unsupported COSE Structure or Header.
const UNSUPPORTED_COSE_STRUCTURE_OR_HEADER: i32 = 2;
/// Unsupported COSE Algorithm.
const UNSUPPORTED_COSE_ALGORITHM: i32 = 3;
/// Signature / MAC verification failed.
const SIGNATURE_MAC_VERIFICATION_FAILED: i32 = 4;
/// Unsupported SUIT Command.
const UNSUPPORTED_SUIT_COMMAND: i32 = 5;
/// Unsupported SUIT Component.
const UNSUPPORTED_SUIT_COMPONENT: i32 = 6;
/// Unauthorized SUIT Component.
const UNAUTHORIZED_SUIT_COMPONENT: i32 = 7;
/// Unsupported SUIT Parameter.
const UNSUPPORTED_SUIT_PARAMETER: i32 = 8;
/// Severing Unsupported.
const SEVERING_UNSUPPORTED: i32 = 9;
/// Condition Failed.
const CONDITION_FAILED: i32 = 10;
/// Operation Failed.
const OPERATION_FAILED: i32 = 11;
/// Invocation Pending.
const INVOCATION_PENDING: i32 = 12;

/// A SUIT Report Reason label.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ReportReason(i32);

impl ReportReason {
    /// Result OK.
    pub const RESULT_OK: Self = Self(RESULT_OK);
    /// CBOR Parse Failure.
    pub const CBOR_PARSE_FAILURE: Self = Self(CBOR_PARSE_FAILURE);
    /// Unsupported COSE Structure or Header.
    pub const UNSUPPORTED_COSE_STRUCTURE_OR_HEADER: Self = Self(UNSUPPORTED_COSE_STRUCTURE_OR_HEADER);
    /// Unsupported COSE Algorithm.
    pub const UNSUPPORTED_COSE_ALGORITHM: Self = Self(UNSUPPORTED_COSE_ALGORITHM);
    /// Signature / MAC verification failed.
    pub const SIGNATURE_MAC_VERIFICATION_FAILED: Self = Self(SIGNATURE_MAC_VERIFICATION_FAILED);
    /// Unsupported SUIT Command.
    pub const UNSUPPORTED_SUIT_COMMAND: Self = Self(UNSUPPORTED_SUIT_COMMAND);
    /// Unsupported SUIT Component.
    pub const UNSUPPORTED_SUIT_COMPONENT: Self = Self(UNSUPPORTED_SUIT_COMPONENT);
    /// Unauthorized SUIT Component.
    pub const UNAUTHORIZED_SUIT_COMPONENT: Self = Self(UNAUTHORIZED_SUIT_COMPONENT);
    /// Unsupported SUIT Parameter.
    pub const UNSUPPORTED_SUIT_PARAMETER: Self = Self(UNSUPPORTED_SUIT_PARAMETER);
    /// Severing Unsupported.
    pub const SEVERING_UNSUPPORTED: Self = Self(SEVERING_UNSUPPORTED);
    /// Condition Failed.
    pub const CONDITION_FAILED: Self = Self(CONDITION_FAILED);
    /// Operation Failed.
    pub const OPERATION_FAILED: Self = Self(OPERATION_FAILED);
    /// Invocation Pending.
    pub const INVOCATION_PENDING: Self = Self(INVOCATION_PENDING);

    /// Returns the raw numeric label.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.0
    }
}

impl From<ReportReason> for i32 {
    fn from(value: ReportReason) -> Self {
        value.0
    }
}

impl TryFrom<i32> for ReportReason {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if is_known(value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

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
