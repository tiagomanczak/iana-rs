//! CBOR Simple Values (IANA registry: cbor-simple-values/simple.csv).
//!
//! Reference: RFC 8949.

/// CBOR Simple Value labels.
/// False.
pub const FALSE: u64 = 20;
/// True.
pub const TRUE: u64 = 21;
/// Null.
pub const NULL: u64 = 22;
/// Undefined.
pub const UNDEFINED: u64 = 23;
/// This value as a map key indicates that the Claim Value is an array of redacted Claim Keys at the same level as the map key. (TEMPORARY - registered 2025-12-16, expires 2026-12-16).
pub const SIMPLE_VALUE_59: u64 = 59;

/// Returns `true` if `value` is a currently assigned CBOR simple value.
#[must_use]
pub const fn is_known(value: u64) -> bool {
    matches!(value, FALSE | TRUE | NULL | UNDEFINED | SIMPLE_VALUE_59)
}
