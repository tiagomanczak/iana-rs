//! CBOR Time Tag Map Keys (IANA registry: cbor-tags/time-tag-map-keys.csv).
//!
//! Reference: RFC 9581.

/// CBOR Time Tag Map Key labels.
/// attoseconds.
pub const ATTOSECONDS: i128 = -18;
/// femtoseconds.
pub const FEMTOSECONDS: i128 = -15;
/// timescale (elective).
pub const TIMESCALE_ELECTIVE: i128 = -13;
/// picoseconds.
pub const PICOSECONDS: i128 = -12;
/// IXDTF Suffix Information (elective).
pub const IXDTF_SUFFIX_INFORMATION_ELECTIVE: i128 = -11;
/// IXDTF Time Zone Hint (elective).
pub const IXDTF_TIME_ZONE_HINT_ELECTIVE: i128 = -10;
/// nanoseconds.
pub const NANOSECONDS: i128 = -9;
/// Guarantee.
pub const GUARANTEE: i128 = -8;
/// Uncertainty.
pub const UNCERTAINTY: i128 = -7;
/// microseconds.
pub const MICROSECONDS: i128 = -6;
/// Offset-Scaled Log Variance.
pub const OFFSET_SCALED_LOG_VARIANCE: i128 = -5;
/// Clock Accuracy.
pub const CLOCK_ACCURACY: i128 = -4;
/// milliseconds.
pub const MILLISECONDS: i128 = -3;
/// Clock Class.
pub const CLOCK_CLASS: i128 = -2;
/// timescale (elective) legacy.
pub const TIMESCALE_ELECTIVE_LEGACY: i128 = -1;
/// base time value as in CBOR Tag 1.
pub const BASE_TIME_VALUE_AS_IN_CBOR_TAG_1: i128 = 1;
/// base time value as in CBOR Tag 4.
pub const BASE_TIME_VALUE_AS_IN_CBOR_TAG_4: i128 = 4;
/// base time value as in CBOR Tag 5.
pub const BASE_TIME_VALUE_AS_IN_CBOR_TAG_5: i128 = 5;
/// IXDTF Time Zone Hint (critical).
pub const IXDTF_TIME_ZONE_HINT_CRITICAL: i128 = 10;
/// IXDTF Suffix Information (critical).
pub const IXDTF_SUFFIX_INFORMATION_CRITICAL: i128 = 11;
/// timescale (critical).
pub const TIMESCALE_CRITICAL: i128 = 13;

/// Returns `true` if `value` is a currently assigned CBOR time tag map key.
#[must_use]
pub const fn is_known(value: i128) -> bool {
    matches!(
        value,
        ATTOSECONDS
            | FEMTOSECONDS
            | TIMESCALE_ELECTIVE
            | PICOSECONDS
            | IXDTF_SUFFIX_INFORMATION_ELECTIVE
            | IXDTF_TIME_ZONE_HINT_ELECTIVE
            | NANOSECONDS
            | GUARANTEE
            | UNCERTAINTY
            | MICROSECONDS
            | OFFSET_SCALED_LOG_VARIANCE
            | CLOCK_ACCURACY
            | MILLISECONDS
            | CLOCK_CLASS
            | TIMESCALE_ELECTIVE_LEGACY
            | BASE_TIME_VALUE_AS_IN_CBOR_TAG_1
            | BASE_TIME_VALUE_AS_IN_CBOR_TAG_4
            | BASE_TIME_VALUE_AS_IN_CBOR_TAG_5
            | IXDTF_TIME_ZONE_HINT_CRITICAL
            | IXDTF_SUFFIX_INFORMATION_CRITICAL
            | TIMESCALE_CRITICAL
    )
}
