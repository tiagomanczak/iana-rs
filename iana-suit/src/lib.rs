//! IANA SUIT (Software Update for the Internet of Things) registry definitions.
//!
//! All numeric labels are `i32`, matching the currently assigned signed integer range
//! used by SUIT CBOR maps.
//!
//! References: RFC-ietf-suit-manifest-34, RFC-ietf-suit-trust-domains-12,
//! RFC-ietf-suit-report-19, RFC-ietf-suit-firmware-encryption-24, and
//! RFC-ietf-suit-mud-10.

#![no_std]
#![forbid(unsafe_code)]

pub mod capability_report;
pub mod commands;
pub mod common;
pub mod component_text;
pub mod envelope;
pub mod manifest;
pub mod parameters;
pub mod record;
pub mod report;
pub mod report_reasons;
pub mod text;

/// Date of the IANA SUIT registry snapshot this crate was last synchronised against.
pub const IANA_SNAPSHOT: &str = "2026-02-17";

#[cfg(test)]
mod tests {
    use super::{
        capability_report, commands, common, component_text, envelope, manifest, parameters,
        record, report, report_reasons, text,
    };

    #[test]
    fn envelope_stable() {
        assert_eq!(envelope::UNSET_DETECTION, 0);
        assert_eq!(envelope::DELEGATION, 1);
        assert_eq!(envelope::AUTHENTICATION_WRAPPER, 2);
        assert_eq!(envelope::MANIFEST, 3);
        assert_eq!(envelope::DEPENDENCY_RESOLUTION, 15);
        assert_eq!(envelope::PAYLOAD_FETCH, 16);
        assert_eq!(envelope::CANDIDATE_VERIFICATION, 18);
        assert_eq!(envelope::PAYLOAD_INSTALLATION, 20);
        assert_eq!(envelope::TEXT_DESCRIPTION, 23);
        assert_eq!(envelope::tag::SUIT_ENVELOPE, 107);
    }

    #[test]
    fn manifest_stable() {
        assert_eq!(manifest::UNSET_DETECTION, 0);
        assert_eq!(manifest::ENCODING_VERSION, 1);
        assert_eq!(manifest::SEQUENCE_NUMBER, 2);
        assert_eq!(manifest::COMMON_DATA, 3);
        assert_eq!(manifest::REFERENCE_URI, 4);
        assert_eq!(manifest::MANIFEST_COMPONENT_ID, 5);
        assert_eq!(manifest::IMAGE_VALIDATION, 7);
        assert_eq!(manifest::IMAGE_LOADING, 8);
        assert_eq!(manifest::IMAGE_INVOCATION, 9);
        assert_eq!(manifest::DEPENDENCY_RESOLUTION, 15);
        assert_eq!(manifest::PAYLOAD_FETCH, 16);
        assert_eq!(manifest::CANDIDATE_VERIFICATION, 18);
        assert_eq!(manifest::PAYLOAD_INSTALLATION, 20);
        assert_eq!(manifest::TEXT_DESCRIPTION, 23);
        assert_eq!(manifest::UNINSTALL, 24);
        assert_eq!(manifest::MANUFACTURER_USAGE_DESCRIPTION, 25);
    }

    #[test]
    fn common_stable() {
        assert_eq!(common::UNSET_DETECTION, 0);
        assert_eq!(common::DEPENDENCIES, 1);
        assert_eq!(common::COMPONENT_IDENTIFIERS, 2);
        assert_eq!(common::COMMON_COMMAND_SEQUENCE, 4);
    }

    #[test]
    fn commands_stable() {
        assert_eq!(commands::UNSET_DETECTION, 0);
        assert_eq!(commands::VENDOR_IDENTIFIER, 1);
        assert_eq!(commands::CLASS_IDENTIFIER, 2);
        assert_eq!(commands::IMAGE_MATCH, 3);
        assert_eq!(commands::COMPONENT_SLOT, 5);
        assert_eq!(commands::CHECK_CONTENT, 6);
        assert_eq!(commands::DEPENDENCY_INTEGRITY, 7);
        assert_eq!(commands::IS_DEPENDENCY, 8);
        assert_eq!(commands::PROCESS_DEPENDENCY, 11);
        assert_eq!(commands::SET_COMPONENT_INDEX, 12);
        assert_eq!(commands::ABORT, 14);
        assert_eq!(commands::TRY_EACH, 15);
        assert_eq!(commands::WRITE_CONTENT, 18);
        assert_eq!(commands::SET_PARAMETERS, 19);
        assert_eq!(commands::OVERRIDE_PARAMETERS, 20);
        assert_eq!(commands::FETCH, 21);
        assert_eq!(commands::COPY, 22);
        assert_eq!(commands::INVOKE, 23);
        assert_eq!(commands::DEVICE_IDENTIFIER, 24);
        assert_eq!(commands::SWAP, 31);
        assert_eq!(commands::RUN_SEQUENCE, 32);
        assert_eq!(commands::UNLINK, 33);
    }

    #[test]
    fn parameters_stable() {
        assert_eq!(parameters::UNSET_DETECTION, 0);
        assert_eq!(parameters::VENDOR_ID, 1);
        assert_eq!(parameters::CLASS_ID, 2);
        assert_eq!(parameters::IMAGE_DIGEST, 3);
        assert_eq!(parameters::COMPONENT_SLOT, 5);
        assert_eq!(parameters::STRICT_ORDER, 12);
        assert_eq!(parameters::SOFT_FAILURE, 13);
        assert_eq!(parameters::IMAGE_SIZE, 14);
        assert_eq!(parameters::CONTENT, 18);
        assert_eq!(parameters::ENCRYPTION_INFO, 19);
        assert_eq!(parameters::URI, 21);
        assert_eq!(parameters::SOURCE_COMPONENT, 22);
        assert_eq!(parameters::INVOKE_ARGS, 23);
        assert_eq!(parameters::DEVICE_ID, 24);
    }

    #[test]
    fn text_stable() {
        assert_eq!(text::UNSET_DETECTION, 0);
        assert_eq!(text::MANIFEST_DESCRIPTION, 1);
        assert_eq!(text::UPDATE_DESCRIPTION, 2);
        assert_eq!(text::MANIFEST_JSON_SOURCE, 3);
        assert_eq!(text::MANIFEST_YAML_SOURCE, 4);
    }

    #[test]
    fn component_text_stable() {
        assert_eq!(component_text::UNSET_DETECTION, 0);
        assert_eq!(component_text::VENDOR_NAME, 1);
        assert_eq!(component_text::MODEL_NAME, 2);
        assert_eq!(component_text::VENDOR_DOMAIN, 3);
        assert_eq!(component_text::MODEL_INFO, 4);
        assert_eq!(component_text::COMPONENT_DESCRIPTION, 5);
        assert_eq!(component_text::COMPONENT_VERSION, 6);
    }

    #[test]
    fn report_stable() {
        assert_eq!(report::NONCE, 2);
        assert_eq!(report::RECORDS, 3);
        assert_eq!(report::RESULT, 4);
        assert_eq!(report::RESULT_CODE, 5);
        assert_eq!(report::RESULT_RECORD, 6);
        assert_eq!(report::RESULT_REASON, 7);
        assert_eq!(report::CAPABILITY_REPORT, 8);
        assert_eq!(report::REFERENCE, 99);
    }

    #[test]
    fn record_stable() {
        assert_eq!(record::MANIFEST_ID, 0);
        assert_eq!(record::MANIFEST_SECTION, 1);
        assert_eq!(record::SECTION_OFFSET, 2);
        assert_eq!(record::COMPONENT_INDEX, 3);
        assert_eq!(record::RECORD_PROPERTIES, 4);
    }

    #[test]
    fn report_reasons_stable() {
        assert_eq!(report_reasons::RESULT_OK, 0);
        assert_eq!(report_reasons::CBOR_PARSE_FAILURE, 1);
        assert_eq!(report_reasons::UNSUPPORTED_COSE_STRUCTURE_OR_HEADER, 2);
        assert_eq!(report_reasons::UNSUPPORTED_COSE_ALGORITHM, 3);
        assert_eq!(report_reasons::SIGNATURE_MAC_VERIFICATION_FAILED, 4);
        assert_eq!(report_reasons::UNSUPPORTED_SUIT_COMMAND, 5);
        assert_eq!(report_reasons::UNSUPPORTED_SUIT_COMPONENT, 6);
        assert_eq!(report_reasons::UNAUTHORIZED_SUIT_COMPONENT, 7);
        assert_eq!(report_reasons::UNSUPPORTED_SUIT_PARAMETER, 8);
        assert_eq!(report_reasons::SEVERING_UNSUPPORTED, 9);
        assert_eq!(report_reasons::CONDITION_FAILED, 10);
        assert_eq!(report_reasons::OPERATION_FAILED, 11);
        assert_eq!(report_reasons::INVOCATION_PENDING, 12);
    }

    #[test]
    fn capability_report_stable() {
        assert_eq!(capability_report::COMPONENTS, 1);
        assert_eq!(capability_report::COMMANDS, 2);
        assert_eq!(capability_report::PARAMETERS, 3);
        assert_eq!(capability_report::CRYPTOGRAPHIC_ALGORITHMS, 4);
        assert_eq!(capability_report::ENVELOPE_ELEMENTS, 5);
        assert_eq!(capability_report::MANIFEST_ELEMENTS, 6);
        assert_eq!(capability_report::COMMON_ELEMENTS, 7);
        assert_eq!(capability_report::TEXT_ELEMENTS, 8);
        assert_eq!(capability_report::COMPONENT_TEXT_ELEMENTS, 9);
        assert_eq!(capability_report::DEPENDENCY_CAPABILITIES, 10);
    }

    #[test]
    fn metadata_matches_package_and_snapshot() {
        assert_eq!(super::IANA_SNAPSHOT, "2026-02-17");
    }
}
