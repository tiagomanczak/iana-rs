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
        use envelope::EnvelopeElement as E;
        assert_eq!(E::DELEGATION.as_i32(), 1);
        assert_eq!(E::AUTHENTICATION_WRAPPER.as_i32(), 2);
        assert_eq!(E::MANIFEST.as_i32(), 3);
        assert_eq!(E::DEPENDENCY_RESOLUTION.as_i32(), 15);
        assert_eq!(E::PAYLOAD_FETCH.as_i32(), 16);
        assert_eq!(E::CANDIDATE_VERIFICATION.as_i32(), 18);
        assert_eq!(E::PAYLOAD_INSTALLATION.as_i32(), 20);
        assert_eq!(E::TEXT_DESCRIPTION.as_i32(), 23);
        assert_eq!(envelope::tag::Tag::SUIT_ENVELOPE.as_i32(), 107);
        assert_eq!(E::try_from(0), Err(0));
    }

    #[test]
    fn manifest_stable() {
        use manifest::ManifestElement as M;
        assert_eq!(M::ENCODING_VERSION.as_i32(), 1);
        assert_eq!(M::SEQUENCE_NUMBER.as_i32(), 2);
        assert_eq!(M::COMMON_DATA.as_i32(), 3);
        assert_eq!(M::REFERENCE_URI.as_i32(), 4);
        assert_eq!(M::MANIFEST_COMPONENT_ID.as_i32(), 5);
        assert_eq!(M::IMAGE_VALIDATION.as_i32(), 7);
        assert_eq!(M::IMAGE_LOADING.as_i32(), 8);
        assert_eq!(M::IMAGE_INVOCATION.as_i32(), 9);
        assert_eq!(M::DEPENDENCY_RESOLUTION.as_i32(), 15);
        assert_eq!(M::PAYLOAD_FETCH.as_i32(), 16);
        assert_eq!(M::CANDIDATE_VERIFICATION.as_i32(), 18);
        assert_eq!(M::PAYLOAD_INSTALLATION.as_i32(), 20);
        assert_eq!(M::TEXT_DESCRIPTION.as_i32(), 23);
        assert_eq!(M::UNINSTALL.as_i32(), 24);
        assert_eq!(M::MANUFACTURER_USAGE_DESCRIPTION.as_i32(), 25);
        assert_eq!(M::try_from(0), Err(0));
    }

    #[test]
    fn common_stable() {
        use common::CommonElement as C;
        assert_eq!(C::DEPENDENCIES.as_i32(), 1);
        assert_eq!(C::COMPONENT_IDENTIFIERS.as_i32(), 2);
        assert_eq!(C::COMMON_COMMAND_SEQUENCE.as_i32(), 4);
        assert_eq!(C::try_from(0), Err(0));
    }

    #[test]
    fn commands_stable() {
        use commands::Command as C;
        assert_eq!(C::VENDOR_IDENTIFIER.as_i32(), 1);
        assert_eq!(C::CLASS_IDENTIFIER.as_i32(), 2);
        assert_eq!(C::IMAGE_MATCH.as_i32(), 3);
        assert_eq!(C::COMPONENT_SLOT.as_i32(), 5);
        assert_eq!(C::CHECK_CONTENT.as_i32(), 6);
        assert_eq!(C::DEPENDENCY_INTEGRITY.as_i32(), 7);
        assert_eq!(C::IS_DEPENDENCY.as_i32(), 8);
        assert_eq!(C::PROCESS_DEPENDENCY.as_i32(), 11);
        assert_eq!(C::SET_COMPONENT_INDEX.as_i32(), 12);
        assert_eq!(C::ABORT.as_i32(), 14);
        assert_eq!(C::TRY_EACH.as_i32(), 15);
        assert_eq!(C::WRITE_CONTENT.as_i32(), 18);
        assert_eq!(C::SET_PARAMETERS.as_i32(), 19);
        assert_eq!(C::OVERRIDE_PARAMETERS.as_i32(), 20);
        assert_eq!(C::FETCH.as_i32(), 21);
        assert_eq!(C::COPY.as_i32(), 22);
        assert_eq!(C::INVOKE.as_i32(), 23);
        assert_eq!(C::DEVICE_IDENTIFIER.as_i32(), 24);
        assert_eq!(C::SWAP.as_i32(), 31);
        assert_eq!(C::RUN_SEQUENCE.as_i32(), 32);
        assert_eq!(C::UNLINK.as_i32(), 33);
        assert_eq!(C::try_from(0), Err(0));
    }

    #[test]
    fn parameters_stable() {
        use parameters::Parameter as P;
        assert_eq!(P::VENDOR_ID.as_i32(), 1);
        assert_eq!(P::CLASS_ID.as_i32(), 2);
        assert_eq!(P::IMAGE_DIGEST.as_i32(), 3);
        assert_eq!(P::COMPONENT_SLOT.as_i32(), 5);
        assert_eq!(P::STRICT_ORDER.as_i32(), 12);
        assert_eq!(P::SOFT_FAILURE.as_i32(), 13);
        assert_eq!(P::IMAGE_SIZE.as_i32(), 14);
        assert_eq!(P::CONTENT.as_i32(), 18);
        assert_eq!(P::ENCRYPTION_INFO.as_i32(), 19);
        assert_eq!(P::URI.as_i32(), 21);
        assert_eq!(P::SOURCE_COMPONENT.as_i32(), 22);
        assert_eq!(P::INVOKE_ARGS.as_i32(), 23);
        assert_eq!(P::DEVICE_ID.as_i32(), 24);
        assert_eq!(P::try_from(0), Err(0));
    }

    #[test]
    fn text_stable() {
        use text::TextValue as T;
        assert_eq!(T::MANIFEST_DESCRIPTION.as_i32(), 1);
        assert_eq!(T::UPDATE_DESCRIPTION.as_i32(), 2);
        assert_eq!(T::MANIFEST_JSON_SOURCE.as_i32(), 3);
        assert_eq!(T::MANIFEST_YAML_SOURCE.as_i32(), 4);
        assert_eq!(T::try_from(0), Err(0));
    }

    #[test]
    fn component_text_stable() {
        use component_text::ComponentTextValue as C;
        assert_eq!(C::VENDOR_NAME.as_i32(), 1);
        assert_eq!(C::MODEL_NAME.as_i32(), 2);
        assert_eq!(C::VENDOR_DOMAIN.as_i32(), 3);
        assert_eq!(C::MODEL_INFO.as_i32(), 4);
        assert_eq!(C::COMPONENT_DESCRIPTION.as_i32(), 5);
        assert_eq!(C::COMPONENT_VERSION.as_i32(), 6);
        assert_eq!(C::try_from(0), Err(0));
    }

    #[test]
    fn report_stable() {
        use report::ReportElement as R;
        assert_eq!(R::NONCE.as_i32(), 2);
        assert_eq!(R::RECORDS.as_i32(), 3);
        assert_eq!(R::RESULT.as_i32(), 4);
        assert_eq!(R::RESULT_CODE.as_i32(), 5);
        assert_eq!(R::RESULT_RECORD.as_i32(), 6);
        assert_eq!(R::RESULT_REASON.as_i32(), 7);
        assert_eq!(R::CAPABILITY_REPORT.as_i32(), 8);
        assert_eq!(R::REFERENCE.as_i32(), 99);
        assert_eq!(R::try_from(0), Err(0));
    }

    #[test]
    fn record_stable() {
        use record::RecordElement as R;
        assert_eq!(R::MANIFEST_ID.as_i32(), 0);
        assert_eq!(R::MANIFEST_SECTION.as_i32(), 1);
        assert_eq!(R::SECTION_OFFSET.as_i32(), 2);
        assert_eq!(R::COMPONENT_INDEX.as_i32(), 3);
        assert_eq!(R::RECORD_PROPERTIES.as_i32(), 4);
        assert_eq!(R::try_from(5), Err(5));
    }

    #[test]
    fn report_reasons_stable() {
        use report_reasons::ReportReason as R;
        assert_eq!(R::RESULT_OK.as_i32(), 0);
        assert_eq!(R::CBOR_PARSE_FAILURE.as_i32(), 1);
        assert_eq!(R::UNSUPPORTED_COSE_STRUCTURE_OR_HEADER.as_i32(), 2);
        assert_eq!(R::UNSUPPORTED_COSE_ALGORITHM.as_i32(), 3);
        assert_eq!(R::SIGNATURE_MAC_VERIFICATION_FAILED.as_i32(), 4);
        assert_eq!(R::UNSUPPORTED_SUIT_COMMAND.as_i32(), 5);
        assert_eq!(R::UNSUPPORTED_SUIT_COMPONENT.as_i32(), 6);
        assert_eq!(R::UNAUTHORIZED_SUIT_COMPONENT.as_i32(), 7);
        assert_eq!(R::UNSUPPORTED_SUIT_PARAMETER.as_i32(), 8);
        assert_eq!(R::SEVERING_UNSUPPORTED.as_i32(), 9);
        assert_eq!(R::CONDITION_FAILED.as_i32(), 10);
        assert_eq!(R::OPERATION_FAILED.as_i32(), 11);
        assert_eq!(R::INVOCATION_PENDING.as_i32(), 12);
        assert_eq!(R::try_from(13), Err(13));
    }

    #[test]
    fn capability_report_stable() {
        use capability_report::CapabilityReportElement as C;
        assert_eq!(C::COMPONENTS.as_i32(), 1);
        assert_eq!(C::COMMANDS.as_i32(), 2);
        assert_eq!(C::PARAMETERS.as_i32(), 3);
        assert_eq!(C::CRYPTOGRAPHIC_ALGORITHMS.as_i32(), 4);
        assert_eq!(C::ENVELOPE_ELEMENTS.as_i32(), 5);
        assert_eq!(C::MANIFEST_ELEMENTS.as_i32(), 6);
        assert_eq!(C::COMMON_ELEMENTS.as_i32(), 7);
        assert_eq!(C::TEXT_ELEMENTS.as_i32(), 8);
        assert_eq!(C::COMPONENT_TEXT_ELEMENTS.as_i32(), 9);
        assert_eq!(C::DEPENDENCY_CAPABILITIES.as_i32(), 10);
    }

    #[test]
    fn capability_report_try_from() {
        use capability_report::CapabilityReportElement as C;
        assert_eq!(C::try_from(1), Ok(C::COMPONENTS));
        assert_eq!(C::try_from(99), Err(99));
        assert_eq!(i32::from(C::COMPONENTS), 1);
    }

    #[test]
    fn metadata_matches_package_and_snapshot() {
        assert_eq!(super::IANA_SNAPSHOT, "2026-02-17");
    }
}
