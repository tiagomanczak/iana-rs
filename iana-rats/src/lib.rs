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

pub mod cmw_indicators;
pub mod eat_intended_uses;
pub mod metadata;

pub use metadata::version;
pub use metadata::version::{
    IANA_REGISTRY_SNAPSHOT, IANA_SNAPSHOT, PACKAGE_NAME, PACKAGE_VERSION, VERSION,
};

#[cfg(test)]
mod tests {
    #[test]
    fn metadata_matches_package_and_snapshot() {
        assert_eq!(super::PACKAGE_NAME, "iana-rats");
        assert_eq!(super::PACKAGE_VERSION, env!("CARGO_PKG_VERSION"));
        assert_eq!(super::VERSION, super::PACKAGE_VERSION);
        assert_eq!(super::IANA_SNAPSHOT, "2026-07-20");
        assert_eq!(super::IANA_REGISTRY_SNAPSHOT, super::IANA_SNAPSHOT);
    }
}
