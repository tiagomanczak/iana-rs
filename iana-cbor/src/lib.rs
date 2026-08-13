//! IANA Concise Binary Object Representation (CBOR) registry definitions.
//!
//! Nonnegative CBOR values use `u64`. Signed CBOR time-map keys use `i128`
//! because the IANA registry includes values outside the `i64` range.
//!
//! References: RFC 8949 and RFC 9581.

#![no_std]
#![forbid(unsafe_code)]

pub mod simple_values;
pub mod tags;
pub mod time_tag_map_keys;
pub mod timescales;

/// Date of the IANA CBOR registry snapshot this crate was last synchronised against.
pub const IANA_SNAPSHOT: &str = "2026-07-20";

#[cfg(test)]
mod tests {
    #[test]
    fn metadata_matches_package() {
        assert_eq!(super::IANA_SNAPSHOT, "2026-07-20");
    }

    #[test]
    fn simple_values_stable() {
        assert_eq!(super::simple_values::FALSE, 20);
        assert_eq!(super::simple_values::TRUE, 21);
        assert_eq!(super::simple_values::NULL, 22);
        assert_eq!(super::simple_values::UNDEFINED, 23);
        assert_eq!(super::simple_values::SIMPLE_VALUE_59, 59);
    }

    #[test]
    fn timescales_stable() {
        assert_eq!(super::timescales::UTC, 0);
        assert_eq!(super::timescales::TAI, 1);
    }

    #[test]
    fn time_tag_map_keys_stable() {
        assert_eq!(super::time_tag_map_keys::ATTOSECONDS, -18);
        assert_eq!(super::time_tag_map_keys::FEMTOSECONDS, -15);
        assert_eq!(super::time_tag_map_keys::TIMESCALE_ELECTIVE, -13);
        assert_eq!(super::time_tag_map_keys::PICOSECONDS, -12);
        assert_eq!(
            super::time_tag_map_keys::IXDTF_SUFFIX_INFORMATION_ELECTIVE,
            -11
        );
        assert_eq!(super::time_tag_map_keys::IXDTF_TIME_ZONE_HINT_ELECTIVE, -10);
        assert_eq!(super::time_tag_map_keys::NANOSECONDS, -9);
        assert_eq!(super::time_tag_map_keys::GUARANTEE, -8);
        assert_eq!(super::time_tag_map_keys::UNCERTAINTY, -7);
        assert_eq!(super::time_tag_map_keys::MICROSECONDS, -6);
        assert_eq!(super::time_tag_map_keys::OFFSET_SCALED_LOG_VARIANCE, -5);
        assert_eq!(super::time_tag_map_keys::CLOCK_ACCURACY, -4);
        assert_eq!(super::time_tag_map_keys::MILLISECONDS, -3);
        assert_eq!(super::time_tag_map_keys::CLOCK_CLASS, -2);
        assert_eq!(super::time_tag_map_keys::TIMESCALE_ELECTIVE_LEGACY, -1);
        assert_eq!(
            super::time_tag_map_keys::BASE_TIME_VALUE_AS_IN_CBOR_TAG_1,
            1
        );
        assert_eq!(
            super::time_tag_map_keys::BASE_TIME_VALUE_AS_IN_CBOR_TAG_4,
            4
        );
        assert_eq!(
            super::time_tag_map_keys::BASE_TIME_VALUE_AS_IN_CBOR_TAG_5,
            5
        );
        assert_eq!(super::time_tag_map_keys::IXDTF_TIME_ZONE_HINT_CRITICAL, 10);
        assert_eq!(
            super::time_tag_map_keys::IXDTF_SUFFIX_INFORMATION_CRITICAL,
            11
        );
        assert_eq!(super::time_tag_map_keys::TIMESCALE_CRITICAL, 13);
    }

    #[test]
    fn suit_related_tags_stable() {
        assert_eq!(super::tags::SUIT_ENVELOPE, 107);
        assert_eq!(super::tags::SUIT_MANIFEST, 1070);
        assert_eq!(super::simple_values::FALSE, 20);
        assert_eq!(super::simple_values::TRUE, 21);
        assert_eq!(super::simple_values::NULL, 22);
        assert_eq!(super::simple_values::UNDEFINED, 23);
    }
}
