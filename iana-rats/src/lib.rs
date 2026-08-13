//! IANA RATS (Remote Attestation Procedures) registry definitions.
//!
//! Registry labels will be added from the official IANA CSV sources while
//! preserving this module structure.
//!
//! Current registries:
//! - Entity Attestation Token (EAT) Intended Uses
//! - RATS Conceptual Message Wrapper (CMW) Indicators
//!
//! Reference: RFC 9711 and the current IANA RATS assignment.

#![no_std]
#![forbid(unsafe_code)]

pub mod cmw_indicators;
pub mod eat_intended_uses;
/// Date of the IANA RATS registry snapshot this crate was last synchronised against.
pub const IANA_SNAPSHOT: &str = "2026-07-20";

#[cfg(test)]
mod tests {
    #[test]
    fn metadata_matches_package_and_snapshot() {
        assert_eq!(super::IANA_SNAPSHOT, "2026-07-20");
    }
}
