//! CBOR Timescales (IANA registry: cbor-tags/timescales.csv).
//!
//! Reference: RFC 9581.

/// CBOR Timescale values.
/// UTC.
pub const UTC: u64 = 0;
/// TAI.
pub const TAI: u64 = 1;

/// Returns `true` if `value` is a currently assigned CBOR timescale.
#[must_use]
pub const fn is_known(value: u64) -> bool {
    matches!(value, UTC | TAI)
}
