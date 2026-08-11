//! IANA SUIT (Software Update for the Internet of Things) registry definitions.
//!
//! All numeric labels are `i64` to accommodate the full signed integer range
//! used by SUIT CBOR maps.
//!
//! References: RFC-ietf-suit-manifest-34, RFC-ietf-suit-trust-domains-12,
//! RFC-ietf-suit-report-19, RFC-ietf-suit-firmware-encryption-24, and
//! RFC-ietf-suit-mud-10.

#![no_std]

pub mod capability_report;
pub mod commands;
pub mod common;
pub mod component_text;
pub mod envelope;
pub mod manifest;
pub mod metadata;
pub mod parameters;
pub mod record;
pub mod report;
pub mod report_reasons;
pub mod text;

pub use metadata::version;
pub use metadata::version::{
    IANA_REGISTRY_SNAPSHOT, IANA_SNAPSHOT, PACKAGE_NAME, PACKAGE_VERSION, VERSION,
};

#[cfg(test)]
mod tests {
    use super::{
        capability_report, commands, common, component_text, envelope, manifest, parameters,
        record, report, report_reasons, text,
    };

    #[test]
    fn registry_constants_remain_stable() {
        assert_eq!(capability_report::label::DEPENDENCY_CAPABILITIES, 10);
        assert_eq!(envelope::label::MANIFEST, 3);
        assert_eq!(manifest::label::MANIFEST_COMPONENT_ID, 5);
        assert_eq!(common::label::COMMON_COMMAND_SEQUENCE, 4);
        assert_eq!(commands::label::WRITE_CONTENT, 18);
        assert_eq!(parameters::label::IMAGE_DIGEST, 3);
        assert_eq!(component_text::label::COMPONENT_VERSION, 6);
        assert_eq!(record::label::RECORD_PROPERTIES, 4);
        assert_eq!(report::label::REFERENCE, 99);
        assert_eq!(report_reasons::label::INVOCATION_PENDING, 12);
        assert_eq!(text::label::MANIFEST_YAML_SOURCE, 4);
    }

    #[test]
    fn metadata_matches_package_and_snapshot() {
        assert_eq!(super::PACKAGE_NAME, "iana-suit");
        assert_eq!(super::PACKAGE_VERSION, env!("CARGO_PKG_VERSION"));
        assert_eq!(super::VERSION, super::PACKAGE_VERSION);
        assert_eq!(super::IANA_SNAPSHOT, "2026-02-17");
        assert_eq!(super::IANA_REGISTRY_SNAPSHOT, super::IANA_SNAPSHOT);
    }
}
