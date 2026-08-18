//! SUIT Report Elements (IANA registry: suit-report-elements).
//!
//! Reference: [RFC-ietf-suit-report-19](https://www.iana.org/go/draft-ietf-suit-report-19)

const NONCE: i32 = 2;
/// Records.
const RECORDS: i32 = 3;
/// Result.
const RESULT: i32 = 4;
/// Result Code.
const RESULT_CODE: i32 = 5;
/// Result Record.
const RESULT_RECORD: i32 = 6;
/// Result Reason.
const RESULT_REASON: i32 = 7;
/// Capability Report.
const CAPABILITY_REPORT: i32 = 8;
/// Reference.
const REFERENCE: i32 = 99;

/// A SUIT Report Element label.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ReportElement(i32);

impl ReportElement {
    /// Nonce.
    pub const NONCE: Self = Self(NONCE);
    /// Records.
    pub const RECORDS: Self = Self(RECORDS);
    /// Result.
    pub const RESULT: Self = Self(RESULT);
    /// Result Code.
    pub const RESULT_CODE: Self = Self(RESULT_CODE);
    /// Result Record.
    pub const RESULT_RECORD: Self = Self(RESULT_RECORD);
    /// Result Reason.
    pub const RESULT_REASON: Self = Self(RESULT_REASON);
    /// Capability Report.
    pub const CAPABILITY_REPORT: Self = Self(CAPABILITY_REPORT);
    /// Reference.
    pub const REFERENCE: Self = Self(REFERENCE);

    /// Returns the raw numeric label.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.0
    }
}

impl From<ReportElement> for i32 {
    fn from(value: ReportElement) -> Self {
        value.0
    }
}

impl TryFrom<i32> for ReportElement {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if is_known(value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

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
