//! CBOR Tags (IANA registry: cbor-tags/tags.csv).
//!
//! Reference: RFC 8949.

/// Standard date/time string.
///
/// IANA tag: `0`
/// IANA semantics: `Standard date/time string; see Section 3.4.1`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const DATE_TIME_STRING: u64 = 0;
/// Epoch-based date/time.
///
/// IANA tag: `1`
/// IANA semantics: `Epoch-based date/time; see Section 3.4.2`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EPOCH_DATE_TIME: u64 = 1;
/// Unsigned bignum.
///
/// IANA tag: `2`
/// IANA semantics: `Unsigned bignum; see Section 3.4.3`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UNSIGNED_BIGNUM: u64 = 2;
/// Negative bignum.
///
/// IANA tag: `3`
/// IANA semantics: `Negative bignum; see Section 3.4.3`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const NEGATIVE_BIGNUM: u64 = 3;
/// Decimal fraction.
///
/// IANA tag: `4`
/// IANA semantics: `Decimal fraction; see Section 3.4.4`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const DECIMAL_FRACTION: u64 = 4;
/// Bigfloat.
///
/// IANA tag: `5`
/// IANA semantics: `Bigfloat; see Section 3.4.4`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const BIGFLOAT: u64 = 5;
/// COSE Single Recipient Encrypted Data Object.
///
/// IANA tag: `16`
/// IANA semantics: `COSE Single Recipient Encrypted Data Object`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const COSE_ENCRYPT0: u64 = 16;
/// COSE Mac w/o Recipients Object.
///
/// IANA tag: `17`
/// IANA semantics: `COSE Mac w/o Recipients Object`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const COSE_MAC0: u64 = 17;
/// COSE Single Signer Data Object.
///
/// IANA tag: `18`
/// IANA semantics: `COSE Single Signer Data Object`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const COSE_SIGN1: u64 = 18;
/// COSE standalone V2 countersignature.
///
/// IANA tag: `19`
/// IANA semantics: `COSE standalone V2 countersignature`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const COSE_COUNTERSIGNATURE: u64 = 19;
/// Expected conversion to base64url encoding.
///
/// IANA tag: `21`
/// IANA semantics: `Expected conversion to base64url encoding; see Section 3.4.5.2`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EXPECTED_BASE64URL: u64 = 21;
/// Expected conversion to base64 encoding.
///
/// IANA tag: `22`
/// IANA semantics: `Expected conversion to base64 encoding; see Section 3.4.5.2`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EXPECTED_BASE64: u64 = 22;
/// Expected conversion to base16 encoding.
///
/// IANA tag: `23`
/// IANA semantics: `Expected conversion to base16 encoding; see Section 3.4.5.2`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EXPECTED_BASE16: u64 = 23;
/// Encoded CBOR data item.
///
/// IANA tag: `24`
/// IANA semantics: `Encoded CBOR data item; see Section 3.4.5.1`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ENCODED_CBOR: u64 = 24;
/// reference the nth previously seen string.
///
/// IANA tag: `25`
/// IANA semantics: `reference the nth previously seen string`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const STRING_REF: u64 = 25;
/// Serialised Perl object with classname and constructor arguments.
///
/// IANA tag: `26`
/// IANA semantics: `Serialised Perl object with classname and constructor arguments`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PERL_OBJECT: u64 = 26;
/// Serialised language-independent object with type name and constructor arguments.
///
/// IANA tag: `27`
/// IANA semantics: `Serialised language-independent object with type name and constructor arguments`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const GENERIC_OBJECT: u64 = 27;
/// mark value as (potentially) shared.
///
/// IANA tag: `28`
/// IANA semantics: `mark value as (potentially) shared`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SHARED_VALUE: u64 = 28;
/// reference nth marked value.
///
/// IANA tag: `29`
/// IANA semantics: `reference nth marked value`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SHARED_VALUE_REF: u64 = 29;
/// Rational number.
///
/// IANA tag: `30`
/// IANA semantics: `Rational number`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const RATIONAL_NUMBER: u64 = 30;
/// Absent value in a CBOR Array.
///
/// IANA tag: `31`
/// IANA semantics: `Absent value in a CBOR Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ABSENT_VALUE_IN_A_CBOR_ARRAY: u64 = 31;
/// URI.
///
/// IANA tag: `32`
/// IANA semantics: `URI; see Section 3.4.5.3`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const URI: u64 = 32;
/// base64url.
///
/// IANA tag: `33`
/// IANA semantics: `base64url; see Section 3.4.5.3`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const BASE64URL: u64 = 33;
/// base64.
///
/// IANA tag: `34`
/// IANA semantics: `base64; see Section 3.4.5.3`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const BASE64: u64 = 34;
/// Regular expression.
///
/// IANA tag: `35`
/// IANA semantics: `Regular expression; see Section 2.4.4.3`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const REGEX: u64 = 35;
/// MIME message.
///
/// IANA tag: `36`
/// IANA semantics: `MIME message; see Section 3.4.5.3`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const MIME_MESSAGE: u64 = 36;
/// Binary UUID (.
///
/// IANA tag: `37`
/// IANA semantics: `Binary UUID (\[RFC9562, Section 4\])`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const BINARY_UUID: u64 = 37;
/// Language-tagged string.
///
/// IANA tag: `38`
/// IANA semantics: `Language-tagged string`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const LANGUAGE_TAGGED_STRING: u64 = 38;
/// Identifier.
///
/// IANA tag: `39`
/// IANA semantics: `Identifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const IDENTIFIER: u64 = 39;
/// Multi-dimensional Array, row-major order.
///
/// IANA tag: `40`
/// IANA semantics: `Multi-dimensional Array, row-major order`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const NDARRAY_ROW_MAJOR: u64 = 40;
/// Homogeneous Array.
///
/// IANA tag: `41`
/// IANA semantics: `Homogeneous Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const HOMOGENEOUS_ARRAY: u64 = 41;
/// IPLD content identifier.
///
/// IANA tag: `42`
/// IANA semantics: `IPLD content identifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const IPLD_CONTENT_IDENTIFIER: u64 = 42;
/// YANG bits datatype.
///
/// IANA tag: `43`
/// IANA semantics: `YANG bits datatype; see Section 6.7.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const YANG_BITS_DATATYPE: u64 = 43;
/// YANG enumeration datatype.
///
/// IANA tag: `44`
/// IANA semantics: `YANG enumeration datatype; see Section 6.6.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const YANG_ENUMERATION_DATATYPE: u64 = 44;
/// YANG identityref datatype.
///
/// IANA tag: `45`
/// IANA semantics: `YANG identityref datatype; see Section 6.10.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const YANG_IDENTITYREF_DATATYPE: u64 = 45;
/// YANG instance-identifier datatype.
///
/// IANA tag: `46`
/// IANA semantics: `YANG instance-identifier datatype; see Section 6.13.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const YANG_INSTANCE_IDENTIFIER_DATATYPE: u64 = 46;
/// YANG Schema Item iDentifier (sid).
///
/// IANA tag: `47`
/// IANA semantics: `YANG Schema Item iDentifier (sid); see Section 3.2.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const YANG_SID: u64 = 47;
/// IEEE MAC Address.
///
/// IANA tag: `48`
/// IANA semantics: `IEEE MAC Address`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const IEEE_MAC_ADDRESS: u64 = 48;
/// IPv4,.
///
/// IANA tag: `52`
/// IANA semantics: `IPv4, \[prefixlen,IPv4\], \[IPv4,prefixpart\]`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const IPV4: u64 = 52;
/// IPv6,.
///
/// IANA tag: `54`
/// IANA semantics: `IPv6, \[prefixlen,IPv6\], \[IPv6,prefixpart\]`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const IPV6: u64 = 54;
/// An array claim element intended to be redacted, or a map key whose key and value are intended to be redacted. (TEMPORARY - registered 2025-12-09, expires 2026-12-09).
///
/// IANA tag: `58`
/// IANA semantics: `An array claim element intended to be redacted, or a map key whose key and value are intended to be redacted. (TEMPORARY - registered 2025-12-09, expires 2026-12-09)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SELECTIVE_DISCLOSURE_ARRAY_CLAIM: u64 = 58;
/// A selective disclosure redacted (array) claim element. (TEMPORARY - registered 2025-12-09, expires 2026-12-09).
///
/// IANA tag: `60`
/// IANA semantics: `A selective disclosure redacted (array) claim element. (TEMPORARY - registered 2025-12-09, expires 2026-12-09)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SELECTIVE_DISCLOSURE_REDACTED_CLAIM: u64 = 60;
/// CBOR Web Token (CWT).
///
/// IANA tag: `61`
/// IANA semantics: `CBOR Web Token (CWT)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CWT: u64 = 61;
/// A marker of a location in a map or an array where a decoy is intended to be inserted. (TEMPORARY - registered 2026-01-27, expires 2027-01-27).
///
/// IANA tag: `62`
/// IANA semantics: `A marker of a location in a map or an array where a decoy is intended to be inserted. (TEMPORARY - registered 2026-01-27, expires 2027-01-27)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const DECOY_MARKER: u64 = 62;
/// Encoded CBOR Sequence.
///
/// IANA tag: `63`
/// IANA semantics: `Encoded CBOR Sequence \[RFC8742\]`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ENCODED_CBOR_SEQUENCE: u64 = 63;
/// uint8 Typed Array.
///
/// IANA tag: `64`
/// IANA semantics: `uint8 Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UINT8_TYPED_ARRAY: u64 = 64;
/// uint16, big endian, Typed Array.
///
/// IANA tag: `65`
/// IANA semantics: `uint16, big endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UINT16_BIG_ENDIAN_TYPED_ARRAY: u64 = 65;
/// uint32, big endian, Typed Array.
///
/// IANA tag: `66`
/// IANA semantics: `uint32, big endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UINT32_BIG_ENDIAN_TYPED_ARRAY: u64 = 66;
/// uint64, big endian, Typed Array.
///
/// IANA tag: `67`
/// IANA semantics: `uint64, big endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UINT64_BIG_ENDIAN_TYPED_ARRAY: u64 = 67;
/// uint8 Typed Array, clamped arithmetic.
///
/// IANA tag: `68`
/// IANA semantics: `uint8 Typed Array, clamped arithmetic`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const TYPED_ARRAY_U8_CLAMPED: u64 = 68;
/// uint16, little endian, Typed Array.
///
/// IANA tag: `69`
/// IANA semantics: `uint16, little endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UINT16_LITTLE_ENDIAN_TYPED_ARRAY: u64 = 69;
/// uint32, little endian, Typed Array.
///
/// IANA tag: `70`
/// IANA semantics: `uint32, little endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UINT32_LITTLE_ENDIAN_TYPED_ARRAY: u64 = 70;
/// uint64, little endian, Typed Array.
///
/// IANA tag: `71`
/// IANA semantics: `uint64, little endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UINT64_LITTLE_ENDIAN_TYPED_ARRAY: u64 = 71;
/// sint8 Typed Array.
///
/// IANA tag: `72`
/// IANA semantics: `sint8 Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SINT8_TYPED_ARRAY: u64 = 72;
/// sint16, big endian, Typed Array.
///
/// IANA tag: `73`
/// IANA semantics: `sint16, big endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SINT16_BIG_ENDIAN_TYPED_ARRAY: u64 = 73;
/// sint32, big endian, Typed Array.
///
/// IANA tag: `74`
/// IANA semantics: `sint32, big endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SINT32_BIG_ENDIAN_TYPED_ARRAY: u64 = 74;
/// sint64, big endian, Typed Array.
///
/// IANA tag: `75`
/// IANA semantics: `sint64, big endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SINT64_BIG_ENDIAN_TYPED_ARRAY: u64 = 75;
/// (reserved).
///
/// IANA tag: `76`
/// IANA semantics: `(reserved)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const RESERVED: u64 = 76;
/// sint16, little endian, Typed Array.
///
/// IANA tag: `77`
/// IANA semantics: `sint16, little endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SINT16_LITTLE_ENDIAN_TYPED_ARRAY: u64 = 77;
/// sint32, little endian, Typed Array.
///
/// IANA tag: `78`
/// IANA semantics: `sint32, little endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SINT32_LITTLE_ENDIAN_TYPED_ARRAY: u64 = 78;
/// sint64, little endian, Typed Array.
///
/// IANA tag: `79`
/// IANA semantics: `sint64, little endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SINT64_LITTLE_ENDIAN_TYPED_ARRAY: u64 = 79;
/// IEEE 754 binary16, big endian, Typed Array.
///
/// IANA tag: `80`
/// IANA semantics: `IEEE 754 binary16, big endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const TYPED_ARRAY_F16_BE: u64 = 80;
/// IEEE 754 binary32, big endian, Typed Array.
///
/// IANA tag: `81`
/// IANA semantics: `IEEE 754 binary32, big endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const TYPED_ARRAY_F32_BE: u64 = 81;
/// IEEE 754 binary64, big endian, Typed Array.
///
/// IANA tag: `82`
/// IANA semantics: `IEEE 754 binary64, big endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const TYPED_ARRAY_F64_BE: u64 = 82;
/// IEEE 754 binary128, big endian, Typed Array.
///
/// IANA tag: `83`
/// IANA semantics: `IEEE 754 binary128, big endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const TYPED_ARRAY_F128_BE: u64 = 83;
/// IEEE 754 binary16, little endian, Typed Array.
///
/// IANA tag: `84`
/// IANA semantics: `IEEE 754 binary16, little endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const TYPED_ARRAY_F16_LE: u64 = 84;
/// IEEE 754 binary32, little endian, Typed Array.
///
/// IANA tag: `85`
/// IANA semantics: `IEEE 754 binary32, little endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const TYPED_ARRAY_F32_LE: u64 = 85;
/// IEEE 754 binary64, little endian, Typed Array.
///
/// IANA tag: `86`
/// IANA semantics: `IEEE 754 binary64, little endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const TYPED_ARRAY_F64_LE: u64 = 86;
/// IEEE 754 binary128, little endian, Typed Array.
///
/// IANA tag: `87`
/// IANA semantics: `IEEE 754 binary128, little endian, Typed Array`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const TYPED_ARRAY_F128_LE: u64 = 87;
/// COSE Encrypted Data Object.
///
/// IANA tag: `96`
/// IANA semantics: `COSE Encrypted Data Object`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const COSE_ENCRYPTED_DATA_OBJECT: u64 = 96;
/// COSE MACed Data Object.
///
/// IANA tag: `97`
/// IANA semantics: `COSE MACed Data Object`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const COSE_MACED_DATA_OBJECT: u64 = 97;
/// COSE Signed Data Object.
///
/// IANA tag: `98`
/// IANA semantics: `COSE Signed Data Object`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const COSE_SIGNED_DATA_OBJECT: u64 = 98;
/// CRI Reference.
///
/// IANA tag: `99`
/// IANA semantics: `CRI Reference`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CRI_REFERENCE: u64 = 99;
/// Number of days since the epoch date 1970-01-01.
///
/// IANA tag: `100`
/// IANA semantics: `Number of days since the epoch date 1970-01-01`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const NUMBER_OF_DAYS_SINCE_THE_EPOCH_DATE: u64 = 100;
/// alternatives as given by the uint + 128.
///
/// IANA tag: `101`
/// IANA semantics: `alternatives as given by the uint + 128; see Section 9.1`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ALTERNATIVE_BASE128: u64 = 101;
/// Geographic Coordinates.
///
/// IANA tag: `103`
/// IANA semantics: `Geographic Coordinates`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const GEOGRAPHIC_COORDINATES: u64 = 103;
/// Geographic Coordinate Reference System WKT or EPSG number.
///
/// IANA tag: `104`
/// IANA semantics: `Geographic Coordinate Reference System WKT or EPSG number`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const GEO_CRS: u64 = 104;
/// SUIT_Envelope as defined in Appendix A of.
///
/// IANA tag: `107`
/// IANA semantics: `SUIT_Envelope as defined in Appendix A of \[RFC-ietf-suit-manifest-34\]`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SUIT_ENVELOPE: u64 = 107;
/// Expected conversion to base16 encoding (lowercase).
///
/// IANA tag: `108`
/// IANA semantics: `Expected conversion to base16 encoding (lowercase)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EXPECTED_BASE16_LOWER: u64 = 108;
/// relative object identifier (BER encoding); SDNV.
///
/// IANA tag: `110`
/// IANA semantics: `relative object identifier (BER encoding); SDNV \[RFC6256\] sequence`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const OID_RELATIVE_BER: u64 = 110;
/// object identifier (BER encoding).
///
/// IANA tag: `111`
/// IANA semantics: `object identifier (BER encoding)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const OID_BER: u64 = 111;
/// object identifier (BER encoding), relative to 1.3.6.1.4.1.
///
/// IANA tag: `112`
/// IANA semantics: `object identifier (BER encoding), relative to 1.3.6.1.4.1`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const OID_BER_RELATIVE_1_3_6_1: u64 = 112;
/// Internet of Things Data Point.
///
/// IANA tag: `120`
/// IANA semantics: `Internet of Things Data Point`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const INTERNET_OF_THINGS_DATA_POINT: u64 = 120;
/// Gordian Envelope.
///
/// IANA tag: `200`
/// IANA semantics: `Gordian Envelope`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const GORDIAN_ENVELOPE: u64 = 200;
/// enclosed dCBOR.
///
/// IANA tag: `201`
/// IANA semantics: `enclosed dCBOR`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ENCLOSED_DCBOR: u64 = 201;
/// mark value as having string references.
///
/// IANA tag: `256`
/// IANA semantics: `mark value as having string references`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const STRING_REFS_MARK: u64 = 256;
/// Binary MIME message.
///
/// IANA tag: `257`
/// IANA semantics: `Binary MIME message`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const BINARY_MIME_MESSAGE: u64 = 257;
/// Mathematical finite set.
///
/// IANA tag: `258`
/// IANA semantics: `Mathematical finite set`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const MATHEMATICAL_FINITE_SET: u64 = 258;
/// Map datatype with key-value operations (e.g. `.get()/.set()/.delete()`).
///
/// IANA tag: `259`
/// IANA semantics: `Map datatype with key-value operations (e.g. `.get()/.set()/.delete()`)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const MAP_WITH_KV_OPS: u64 = 259;
/// Network Address (IPv4 or IPv6 or MAC Address) (DEPRECATED in favor of 52 and 54         for IP addresses).
///
/// IANA tag: `260`
/// IANA semantics: `Network Address (IPv4 or IPv6 or MAC Address) (DEPRECATED in favor of 52 and 54         for IP addresses)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const NETWORK_ADDRESS_IPV4_OR_IPV6_OR_MAC: u64 = 260;
/// Network Address Prefix (IPv4 or IPv6 Address + Mask Length) (DEPRECATED in favor of 52 and 54         for IP addresses).
///
/// IANA tag: `261`
/// IANA semantics: `Network Address Prefix (IPv4 or IPv6 Address + Mask Length) (DEPRECATED in favor of 52 and 54         for IP addresses)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const NETWORK_ADDRESS_PREFIX_IPV4_OR_IPV6: u64 = 261;
/// Embedded JSON Object.
///
/// IANA tag: `262`
/// IANA semantics: `Embedded JSON Object`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EMBEDDED_JSON_OBJECT: u64 = 262;
/// Hexadecimal string.
///
/// IANA tag: `263`
/// IANA semantics: `Hexadecimal string`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const HEXADECIMAL_STRING: u64 = 263;
/// Decimal fraction with arbitrary exponent.
///
/// IANA tag: `264`
/// IANA semantics: `Decimal fraction with arbitrary exponent`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const DECIMAL_FRACTION_EXT: u64 = 264;
/// Bigfloat with arbitrary exponent.
///
/// IANA tag: `265`
/// IANA semantics: `Bigfloat with arbitrary exponent`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const BIGFLOAT_EXT: u64 = 265;
/// Internationalized resource identifier (IRI).
///
/// IANA tag: `266`
/// IANA semantics: `Internationalized resource identifier (IRI)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const IRI: u64 = 266;
/// Internationalized resource identifier reference (IRI reference).
///
/// IANA tag: `267`
/// IANA semantics: `Internationalized resource identifier reference (IRI reference)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const IRI_REF: u64 = 267;
/// Extended decimal fraction.
///
/// IANA tag: `268`
/// IANA semantics: `Extended decimal fraction`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EXTENDED_DECIMAL_FRACTION: u64 = 268;
/// Extended bigfloat.
///
/// IANA tag: `269`
/// IANA semantics: `Extended bigfloat`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EXTENDED_BIGFLOAT: u64 = 269;
/// Extended rational number.
///
/// IANA tag: `270`
/// IANA semantics: `Extended rational number`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EXTENDED_RATIONAL_NUMBER: u64 = 270;
/// DDoS Open Threat Signaling (DOTS) signal channel object, as defined in.
///
/// IANA tag: `271`
/// IANA semantics: `DDoS Open Threat Signaling (DOTS) signal channel object, as defined in \[RFC9132\]`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const DOTS_SIGNAL: u64 = 271;
/// Non-UTF-8 CESU-8 string.
///
/// IANA tag: `272`
/// IANA semantics: `Non-UTF-8 CESU-8 string`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const NON_UTF_8_CESU_8_STRING: u64 = 272;
/// Non-UTF-8 WTF-8 string.
///
/// IANA tag: `273`
/// IANA semantics: `Non-UTF-8 WTF-8 string`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const NON_UTF_8_WTF_8_STRING: u64 = 273;
/// Non-UTF-8 MUTF-8 string.
///
/// IANA tag: `274`
/// IANA semantics: `Non-UTF-8 MUTF-8 string`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const NON_UTF_8_MUTF_8_STRING: u64 = 274;
/// Map contains only keys that are of type Text String (major type 3).
///
/// IANA tag: `275`
/// IANA semantics: `Map contains only keys that are of type Text String (major type 3)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const TEXT_KEY_MAP: u64 = 275;
/// ERIS binary read capability.
///
/// IANA tag: `276`
/// IANA semantics: `ERIS binary read capability`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ERIS_BINARY_READ_CAPABILITY: u64 = 276;
/// Universal Geographical Area Description (GAD) shape.
///
/// IANA tag: `277`
/// IANA semantics: `Universal Geographical Area Description (GAD) shape; see Section 5`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const GAD_SHAPE: u64 = 277;
/// Universal Geographical Area Description (GAD) description of velocity.
///
/// IANA tag: `278`
/// IANA semantics: `Universal Geographical Area Description (GAD) description of velocity; see Section 8`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const GAD_VELOCITY: u64 = 278;
/// Coordinate Reference System Wrapper.
///
/// IANA tag: `279`
/// IANA semantics: `Coordinate Reference System Wrapper`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const COORDINATE_REFERENCE_SYSTEM_WRAPPER: u64 = 279;
/// Symbol.
///
/// IANA tag: `280`
/// IANA semantics: `Symbol`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SYMBOL: u64 = 280;
/// Linked list.
///
/// IANA tag: `281`
/// IANA semantics: `Linked list`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const LINKED_LIST: u64 = 281;
/// Character.
///
/// IANA tag: `282`
/// IANA semantics: `Character`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CHARACTER: u64 = 282;
/// Object.
///
/// IANA tag: `283`
/// IANA semantics: `Object`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const OBJECT: u64 = 283;
/// JSON Numeric Value, Represented as its JSON Text.
///
/// IANA tag: `284`
/// IANA semantics: `JSON Numeric Value, Represented as its JSON Text`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const JSON_NUMBER_TEXT: u64 = 284;
/// SUIT_Report_Protected.
///
/// IANA tag: `285`
/// IANA semantics: `SUIT_Report_Protected`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SUIT_REPORT_PROTECTED: u64 = 285;
/// SUIT_Reference.
///
/// IANA tag: `286`
/// IANA semantics: `SUIT_Reference`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SUIT_REFERENCE: u64 = 286;
/// SUIT_Capability_Report.
///
/// IANA tag: `287`
/// IANA semantics: `SUIT_Capability_Report`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SUIT_CAPABILITY_REPORT: u64 = 287;
/// isolate shared values within this scope.
///
/// IANA tag: `296`
/// IANA semantics: `isolate shared values within this scope`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SHARED_VALUE_SCOPE: u64 = 296;
/// Geohash String.
///
/// IANA tag: `301`
/// IANA semantics: `Geohash String`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const GEOHASH_STRING: u64 = 301;
/// Earmarked for CoRIM.
///
/// IANA tag: `500`
/// IANA semantics: `Earmarked for CoRIM`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EARMARKED_FOR_CORIM: u64 = 500;
/// A CBOR tag that contains a corim-map..
///
/// IANA tag: `501`
/// IANA semantics: `A CBOR tag that contains a corim-map.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CORIM_MAP: u64 = 501;
/// A CBOR tag that contains a conciseswid-tag-map..
///
/// IANA tag: `505`
/// IANA semantics: `A CBOR tag that contains a conciseswid-tag-map.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CONCISE_SWID: u64 = 505;
/// A CBOR tag that contains a concisemid-tag-map..
///
/// IANA tag: `506`
/// IANA semantics: `A CBOR tag that contains a concisemid-tag-map.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CONCISE_MID: u64 = 506;
/// A CBOR tag that contains an xcorim-map..
///
/// IANA tag: `526`
/// IANA semantics: `A CBOR tag that contains an xcorim-map.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const XCORIM_MAP: u64 = 526;
/// A CBOR tag that contains either: xcorimmap, or signed-xcorim..
///
/// IANA tag: `527`
/// IANA semantics: `A CBOR tag that contains either: xcorimmap, or signed-xcorim.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const XCORIM_OR_SIGNED_XCORIM: u64 = 527;
/// A CBOR tag that contains a UEID between 7 and 33 bytes..
///
/// IANA tag: `550`
/// IANA semantics: `A CBOR tag that contains a UEID between 7 and 33 bytes.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UEID: u64 = 550;
/// Earmarked for CoRIM.
///
/// IANA tag: `551`
/// IANA semantics: `Earmarked for CoRIM`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EARMARKED_FOR_CORIM_LABEL_551: u64 = 551;
/// A CBOR tag that contains a security version number that is evaluated with equivalence semantics..
///
/// IANA tag: `552`
/// IANA semantics: `A CBOR tag that contains a security version number that is evaluated with equivalence semantics.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CORIM_SVN: u64 = 552;
/// A CBOR tag that contains min-svn that identifies a security version number that is evaluated with greater than or equals semantics.
///
/// IANA tag: `553`
/// IANA semantics: `A CBOR tag that contains min-svn that identifies a security version number that is evaluated with greater than or equals semantics`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CORIM_MIN_SVN: u64 = 553;
/// A CBOR tag that contains a PEM encoded SubjectPublicKeyInfo. See Section 13 of.
///
/// IANA tag: `554`
/// IANA semantics: `A CBOR tag that contains a PEM encoded SubjectPublicKeyInfo. See Section 13 of \[RFC7468\].`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SUBJECT_PUBLIC_KEY_INFO_PEM: u64 = 554;
/// A CBOR tag that contains a PEM encoded SubjectPublicKeyInfo. See Section 13 of.
///
/// IANA tag: `555`
/// IANA semantics: `A CBOR tag that contains a PEM encoded SubjectPublicKeyInfo. See Section 13 of \[RFC7468\].`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SUBJECT_PUBLIC_KEY_INFO_PEM_B: u64 = 555;
/// A CBOR tag that contains an X.509 certificate chain created by the concatenation of as many PEM encoded X.509 certificates as needed. The certificates MUST be concatenated in order that each directly certifies the one preceding..
///
/// IANA tag: `556`
/// IANA semantics: `A CBOR tag that contains an X.509 certificate chain created by the concatenation of as many PEM encoded X.509 certificates as needed. The certificates MUST be concatenated in order that each directly certifies the one preceding.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const X509_CERT_CHAIN: u64 = 556;
/// A CBOR tag that contains a byte string interpreted as an array of bits..
///
/// IANA tag: `560`
/// IANA semantics: `A CBOR tag that contains a byte string interpreted as an array of bits.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const BIT_ARRAY: u64 = 560;
/// spdm-toc-map.
///
/// IANA tag: `570`
/// IANA semantics: `spdm-toc-map`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SPDM_TOC_MAP: u64 = 570;
/// concise-evidence-map.
///
/// IANA tag: `571`
/// IANA semantics: `concise-evidence-map`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CONCISE_EVIDENCE_MAP: u64 = 571;
/// Unprotected CWT Claims Set.
///
/// IANA tag: `601`
/// IANA semantics: `Unprotected CWT Claims Set \[RFC9781\]`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UNPROTECTED_CWT_CLAIMS_SET: u64 = 601;
/// Detached EAT Bundle.
///
/// IANA tag: `602`
/// IANA semantics: `Detached EAT Bundle`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const DETACHED_EAT_BUNDLE: u64 = 602;
/// extended time.
///
/// IANA tag: `1001`
/// IANA semantics: `extended time`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EXTENDED_TIME: u64 = 1001;
/// duration.
///
/// IANA tag: `1002`
/// IANA semantics: `duration`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const DURATION: u64 = 1002;
/// period.
///
/// IANA tag: `1003`
/// IANA semantics: `period`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PERIOD: u64 = 1003;
/// .
///
/// IANA tag: `1004`
/// IANA semantics: `\[RFC3339\] full-date string`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const TAG_1004: u64 = 1004;
/// Object type identifier.
///
/// IANA tag: `1010`
/// IANA semantics: `Object type identifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const OBJECT_TYPE_IDENTIFIER: u64 = 1010;
/// Multi-dimensional Array, column-major order.
///
/// IANA tag: `1040`
/// IANA semantics: `Multi-dimensional Array, column-major order`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const NDARRAY_COLUMN_MAJOR: u64 = 1040;
/// IEEE OUI/CID.
///
/// IANA tag: `1048`
/// IANA semantics: `IEEE OUI/CID`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const IEEE_OUI_CID: u64 = 1048;
/// SUIT_Manifest as defined in Appendix A of.
///
/// IANA tag: `1070`
/// IANA semantics: `SUIT_Manifest as defined in Appendix A of \[RFC-ietf-suit-manifest-34\]`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SUIT_MANIFEST: u64 = 1070;
/// .
///
/// IANA tag: `18556`
/// IANA semantics: `\[COSE algorithm identifier, Base Hash value\]`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const TAG_18556: u64 = 18556;
/// description of the value instead of the value itself.
///
/// IANA tag: `20853`
/// IANA semantics: `description of the value instead of the value itself`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const VALUE_DESCRIPTION_HINT: u64 = 20853;
/// I-Regexp.
///
/// IANA tag: `21065`
/// IANA semantics: `I-Regexp`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const I_REGEXP: u64 = 21065;
/// ECMAScript RegExp.
///
/// IANA tag: `21066`
/// IANA semantics: `ECMAScript RegExp \[<https://262.ecma-international.org/14.0/#sec-regexp-regular-expression-objects>\]`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ECMASCRIPT_REGEXP: u64 = 21066;
/// (always invalid in interchange) programming aid for simple values.
///
/// IANA tag: `21334`
/// IANA semantics: `(always invalid in interchange) programming aid for simple values`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ALWAYS_INVALID_IN_INTERCHANGE: u64 = 21334;
/// a CBOR Tag identifier.
///
/// IANA tag: `21607`
/// IANA semantics: `a CBOR Tag identifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const A_CBOR_TAG_IDENTIFIER: u64 = 21607;
/// hint that indicates an additional level of indirection.
///
/// IANA tag: `22098`
/// IANA semantics: `hint that indicates an additional level of indirection`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const INDIRECTION_HINT: u64 = 22098;
/// Capture.
///
/// IANA tag: `25441`
/// IANA semantics: `Capture \[3\]`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CAPTURE: u64 = 25441;
/// Identifier for a FHIR constant.
///
/// IANA tag: `32768`
/// IANA semantics: `Identifier for a FHIR constant`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const IDENTIFIER_FOR_A_FHIR_CONSTANT: u64 = 32768;
/// External reference.
///
/// IANA tag: `32769`
/// IANA semantics: `External reference`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EXTERNAL_REFERENCE: u64 = 32769;
/// Logical operator: NONE / NOT. Encodes the logical operation (!item1&&!item2&&!item3&&..., if array), otherwise (!item)..
///
/// IANA tag: `32870`
/// IANA semantics: `Logical operator: NONE / NOT. Encodes the logical operation (!item1&&!item2&&!item3&&..., if array), otherwise (!item).`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const LOGICAL_NONE: u64 = 32870;
/// Logical operator: ANY. Encodes the logical operation (item1||item2||item3||...)..
///
/// IANA tag: `32871`
/// IANA semantics: `Logical operator: ANY. Encodes the logical operation (item1||item2||item3||...).`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const LOGICAL_ANY: u64 = 32871;
/// Logical operator: ALL. Encodes the logical operation (item1&&item2&&item3&&...)..
///
/// IANA tag: `32872`
/// IANA semantics: `Logical operator: ALL. Encodes the logical operation (item1&&item2&&item3&&...).`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const LOGICAL_ALL: u64 = 32872;
/// ur:known-value, Semantic signifier.
///
/// IANA tag: `40000`
/// IANA semantics: `ur:known-value, Semantic signifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_KNOWN_VALUE_SEMANTIC_SIGNIFIER: u64 = 40000;
/// ur:digest, 32-byte SHA-256 digest.
///
/// IANA tag: `40001`
/// IANA semantics: `ur:digest, 32-byte SHA-256 digest`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_DIGEST_32_BYTE_SHA_256_DIGEST: u64 = 40001;
/// ur:encrypted, IETF ChaCha20-Poly1305 (.
///
/// IANA tag: `40002`
/// IANA semantics: `ur:encrypted, IETF ChaCha20-Poly1305 (\[RFC8439\]) encrypted message`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_ENCRYPTED_IETF_CHACHA20_POLY1305: u64 = 40002;
/// ur:compressed,.
///
/// IANA tag: `40003`
/// IANA semantics: `ur:compressed, \[RFC1951\] DEFLATE-compressed message`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_COMPRESSED: u64 = 40003;
/// ur:request, Transaction Request identifier.
///
/// IANA tag: `40004`
/// IANA semantics: `ur:request, Transaction Request identifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_REQUEST_TRANSACTION_REQUEST: u64 = 40004;
/// ur:response, Transaction response identifier.
///
/// IANA tag: `40005`
/// IANA semantics: `ur:response, Transaction response identifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_RESPONSE_TRANSACTION_RESPONSE: u64 = 40005;
/// ur:function, Envelope expression function identifier.
///
/// IANA tag: `40006`
/// IANA semantics: `ur:function, Envelope expression function identifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_FUNCTION_ENVELOPE_EXPRESSION: u64 = 40006;
/// ur:parameter, Envelope expression parameter identifier.
///
/// IANA tag: `40007`
/// IANA semantics: `ur:parameter, Envelope expression parameter identifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_PARAMETER_ENVELOPE_EXPRESSION: u64 = 40007;
/// ur:placeholder, Envelope expression placeholder identifier.
///
/// IANA tag: `40008`
/// IANA semantics: `ur:placeholder, Envelope expression placeholder identifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_PLACEHOLDER_ENVELOPE_EXPRESSION: u64 = 40008;
/// ur:replacement, Envelope expression replacement identifier.
///
/// IANA tag: `40009`
/// IANA semantics: `ur:replacement, Envelope expression replacement identifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_REPLACEMENT_ENVELOPE_EXPRESSION: u64 = 40009;
/// ur:agreement-private-key, Curve25519 private key for X25519 key agreement.
///
/// IANA tag: `40010`
/// IANA semantics: `ur:agreement-private-key, Curve25519 private key for X25519 key agreement`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_AGREEMENT_PRIVATE_KEY_CURVE25519: u64 = 40010;
/// ur:agreement-public-key, Curve25519 public key for X25519 key agreement.
///
/// IANA tag: `40011`
/// IANA semantics: `ur:agreement-public-key, Curve25519 public key for X25519 key agreement`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_AGREEMENT_PUBLIC_KEY_CURVE25519: u64 = 40011;
/// ur:arid, Apparently Random Identifier.
///
/// IANA tag: `40012`
/// IANA semantics: `ur:arid, Apparently Random Identifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_ARID: u64 = 40012;
/// ur:crypto-prvkeys, Private keys for cryptographic operations.
///
/// IANA tag: `40013`
/// IANA semantics: `ur:crypto-prvkeys, Private keys for cryptographic operations`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_CRYPTO_PRVKEYS_PRIVATE_KEYS_FOR: u64 = 40013;
/// ur:nonce, Cryptographic nonce.
///
/// IANA tag: `40014`
/// IANA semantics: `ur:nonce, Cryptographic nonce`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_NONCE_CRYPTOGRAPHIC_NONCE: u64 = 40014;
/// ur:password, Scrypt-hashed password.
///
/// IANA tag: `40015`
/// IANA semantics: `ur:password, Scrypt-hashed password`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_PASSWORD_SCRYPT_HASHED_PASSWORD: u64 = 40015;
/// ur:crypto-prvkeys, Private key base (key material).
///
/// IANA tag: `40016`
/// IANA semantics: `ur:crypto-prvkeys, Private key base (key material)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_CRYPTO_PRVKEYS_BASE: u64 = 40016;
/// ur:crypto-pubkeys, Public key base (signing and agreement public key bundle).
///
/// IANA tag: `40017`
/// IANA semantics: `ur:crypto-pubkeys, Public key base (signing and agreement public key bundle)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_CRYPTO_PUBKEYS_PUBLIC_KEY_BASE: u64 = 40017;
/// ur:salt, Random salt used for hash tree decorrelation.
///
/// IANA tag: `40018`
/// IANA semantics: `ur:salt, Random salt used for hash tree decorrelation`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_SALT: u64 = 40018;
/// ur:crypto-sealed, Encrypted message and ephemeral public key.
///
/// IANA tag: `40019`
/// IANA semantics: `ur:crypto-sealed, Encrypted message and ephemeral public key`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_CRYPTO_SEALED: u64 = 40019;
/// ur:signature, Cryptographic signature.
///
/// IANA tag: `40020`
/// IANA semantics: `ur:signature, Cryptographic signature`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_SIGNATURE: u64 = 40020;
/// ur:signing-private-key, Cryptographic private key used for signing.
///
/// IANA tag: `40021`
/// IANA semantics: `ur:signing-private-key, Cryptographic private key used for signing`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_SIGNING_PRIVATE_KEY: u64 = 40021;
/// ur:signing-public-key, Cryptographic public key used for signing.
///
/// IANA tag: `40022`
/// IANA semantics: `ur:signing-public-key, Cryptographic public key used for signing`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_SIGNING_PUBLIC_KEY_CRYPTOGRAPHIC: u64 = 40022;
/// ur:crypto-key, Cryptographic key used for symmetric encryption.
///
/// IANA tag: `40023`
/// IANA semantics: `ur:crypto-key, Cryptographic key used for symmetric encryption`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_CRYPTO_KEY: u64 = 40023;
/// ur:xid, Extensible identifier or XID Document.
///
/// IANA tag: `40024`
/// IANA semantics: `ur:xid, Extensible identifier or XID Document`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_XID_EXTENSIBLE_IDENTIFIER_OR_XID: u64 = 40024;
/// ur:reference, Cryptographically secure reference to an object.
///
/// IANA tag: `40025`
/// IANA semantics: `ur:reference, Cryptographically secure reference to an object`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_REFERENCE: u64 = 40025;
/// ur:event, Event identifier.
///
/// IANA tag: `40026`
/// IANA semantics: `ur:event, Event identifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_EVENT_EVENT_IDENTIFIER: u64 = 40026;
/// ur:encrypted-key, Content key encrypted with a derivation function.
///
/// IANA tag: `40027`
/// IANA semantics: `ur:encrypted-key, Content key encrypted with a derivation function`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_ENCRYPTED_KEY: u64 = 40027;
/// ur:mlkem-private-key, Private key for MLKEM key encapsulation.
///
/// IANA tag: `40100`
/// IANA semantics: `ur:mlkem-private-key, Private key for MLKEM key encapsulation`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_MLKEM_PRIVATE_KEY: u64 = 40100;
/// ur:mlkem-public-key, Public key for MLKEM key encapsulation.
///
/// IANA tag: `40101`
/// IANA semantics: `ur:mlkem-public-key, Public key for MLKEM key encapsulation`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_MLKEM_PUBLIC_KEY_PUBLIC_KEY_FOR: u64 = 40101;
/// ur:mlkem-ciphertext, Ciphertext for MLKEM key encapsulation.
///
/// IANA tag: `40102`
/// IANA semantics: `ur:mlkem-ciphertext, Ciphertext for MLKEM key encapsulation`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_MLKEM_CIPHERTEXT_CIPHERTEXT_FOR: u64 = 40102;
/// ur:mldsa-private-key, Private key for MLDSA signature generation.
///
/// IANA tag: `40103`
/// IANA semantics: `ur:mldsa-private-key, Private key for MLDSA signature generation`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_MLDSA_PRIVATE_KEY: u64 = 40103;
/// ur:mldsa-public-key, Public key for MLDSA signature verification.
///
/// IANA tag: `40104`
/// IANA semantics: `ur:mldsa-public-key, Public key for MLDSA signature verification`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_MLDSA_PUBLIC_KEY_PUBLIC_KEY_FOR: u64 = 40104;
/// ur:mldsa-signature, MLDSA signature.
///
/// IANA tag: `40105`
/// IANA semantics: `ur:mldsa-signature, MLDSA signature`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_MLDSA_SIGNATURE_MLDSA_SIGNATURE: u64 = 40105;
/// ur:seed, Cryptographic seed.
///
/// IANA tag: `40300`
/// IANA semantics: `ur:seed, Cryptographic seed`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_SEED_CRYPTOGRAPHIC_SEED: u64 = 40300;
/// ur:hdkey, Bitcoin BIP-32 HD key.
///
/// IANA tag: `40303`
/// IANA semantics: `ur:hdkey, Bitcoin BIP-32 HD key`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_HDKEY_BITCOIN_BIP_32_HD_KEY: u64 = 40303;
/// ur:keypath, Bitcoin BIP-32 key derivation path.
///
/// IANA tag: `40304`
/// IANA semantics: `ur:keypath, Bitcoin BIP-32 key derivation path`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_KEYPATH_BITCOIN_BIP_32_KEY: u64 = 40304;
/// ur:coin-info, Cryptographic asset and network specifier.
///
/// IANA tag: `40305`
/// IANA semantics: `ur:coin-info, Cryptographic asset and network specifier`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_COIN_INFO: u64 = 40305;
/// ur:eckey, Bitcoin elliptic curve key (private or public).
///
/// IANA tag: `40306`
/// IANA semantics: `ur:eckey, Bitcoin elliptic curve key (private or public)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_ECKEY_BITCOIN_ELLIPTIC_CURVE_KEY: u64 = 40306;
/// ur:address, Cryptocurrency address.
///
/// IANA tag: `40307`
/// IANA semantics: `ur:address, Cryptocurrency address`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_ADDRESS_CRYPTOCURRENCY_ADDRESS: u64 = 40307;
/// ur:output-descriptor, Bitcoin output descriptor.
///
/// IANA tag: `40308`
/// IANA semantics: `ur:output-descriptor, Bitcoin output descriptor`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_OUTPUT_DESCRIPTOR_BITCOIN_OUTPUT: u64 = 40308;
/// ur:sskr, Sharded Secret Key Reconstruction (SSKR) share.
///
/// IANA tag: `40309`
/// IANA semantics: `ur:sskr, Sharded Secret Key Reconstruction (SSKR) share`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_SSKR_SHARDED_SECRET_KEY: u64 = 40309;
/// ur:psbt, Partially Signed Bitcoin Transaction.
///
/// IANA tag: `40310`
/// IANA semantics: `ur:psbt, Partially Signed Bitcoin Transaction`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_PSBT_PARTIALLY_SIGNED_BITCOIN: u64 = 40310;
/// ur:account-descriptor, Bitcoin account descriptor.
///
/// IANA tag: `40311`
/// IANA semantics: `ur:account-descriptor, Bitcoin account descriptor`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_ACCOUNT_DESCRIPTOR: u64 = 40311;
/// ur:ssh-private, Text format SSH private key.
///
/// IANA tag: `40800`
/// IANA semantics: `ur:ssh-private, Text format SSH private key`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_SSH_PRIVATE: u64 = 40800;
/// ur:ssh-public, Text format SSH public key.
///
/// IANA tag: `40801`
/// IANA semantics: `ur:ssh-public, Text format SSH public key`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_SSH_PUBLIC: u64 = 40801;
/// ur:ssh-signature, Text format SSH signature.
///
/// IANA tag: `40802`
/// IANA semantics: `ur:ssh-signature, Text format SSH signature`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_SSH_SIGNATURE_TEXT_FORMAT_SSH: u64 = 40802;
/// ur:ssh-certificate, Text format SSH certificate.
///
/// IANA tag: `40803`
/// IANA semantics: `ur:ssh-certificate, Text format SSH certificate`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_SSH_CERTIFICATE_TEXT_FORMAT_SSH: u64 = 40803;
/// Concordium smart contract address.
///
/// IANA tag: `40919`
/// IANA semantics: `Concordium smart contract address`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CONCORDIUM_SMART_CONTRACT_ADDRESS: u64 = 40919;
/// Fraction.
///
/// IANA tag: `41728`
/// IANA semantics: `Fraction`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const FRACTION: u64 = 41728;
/// Fraction (-NaN signals).
///
/// IANA tag: `41729`
/// IANA semantics: `Fraction (-NaN signals)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const FRACTION_NAN_SIGNALS: u64 = 41729;
/// Fraction (+NaN signals).
///
/// IANA tag: `41730`
/// IANA semantics: `Fraction (+NaN signals)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const FRACTION_NAN_SIGNALS_LABEL_41730: u64 = 41730;
/// Fraction (Both NaNs signal).
///
/// IANA tag: `41731`
/// IANA semantics: `Fraction (Both NaNs signal)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const FRACTION_BOTH_NANS_SIGNAL: u64 = 41731;
/// A confidentiality clearance. The key value pairs of the map are defined in ADatP-4774.8.
///
/// IANA tag: `42600`
/// IANA semantics: `A confidentiality clearance. The key value pairs of the map are defined in ADatP-4774.8`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const A_CONFIDENTIALITY_CLEARANCE_THE_KEY: u64 = 42600;
/// A metadata binding. The elements of the array are defined in AdatP-4778.8. The tag is also used as part of the magic number in on-disk detached and encapsulating bindings..
///
/// IANA tag: `42601`
/// IANA semantics: `A metadata binding. The elements of the array are defined in AdatP-4778.8. The tag is also used as part of the magic number in on-disk detached and encapsulating bindings.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const NCMS_METADATA_BINDING: u64 = 42601;
/// A collection of NCMS metadata elements. The key value pairs of the map are defined in AdatP-5636.8.
///
/// IANA tag: `42602`
/// IANA semantics: `A collection of NCMS metadata elements. The key value pairs of the map are defined in AdatP-5636.8`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const NCMS_METADATA_COLLECTION: u64 = 42602;
/// Single complex number: array elements are real (I) and imaginary (Q) components.
///
/// IANA tag: `43000`
/// IANA semantics: `Single complex number: array elements are real (I) and imaginary (Q) components`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const COMPLEX_NUMBER: u64 = 43000;
/// Array of complex numbers in interleaved form: complex value k is stored with real (I) part  at array index 2k and imaginary (Q) part at index (2k + 1).
///
/// IANA tag: `43001`
/// IANA semantics: `Array of complex numbers in interleaved form: complex value k is stored with real (I) part  at array index 2k and imaginary (Q) part at index (2k + 1)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const COMPLEX_NUMBER_ARRAY: u64 = 43001;
/// PlatformV_IS_ID.
///
/// IANA tag: `50000`
/// IANA semantics: `PlatformV_IS_ID`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PLATFORMV_IS_ID: u64 = 50000;
/// PlatformV_IS_NAME.
///
/// IANA tag: `50001`
/// IANA semantics: `PlatformV_IS_NAME`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PLATFORMV_IS_NAME: u64 = 50001;
/// PlatformV_IS_VALUE.
///
/// IANA tag: `50002`
/// IANA semantics: `PlatformV_IS_VALUE`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PLATFORMV_IS_VALUE: u64 = 50002;
/// PlatformV_HAS_COMPOSITE_VALUE.
///
/// IANA tag: `50003`
/// IANA semantics: `PlatformV_HAS_COMPOSITE_VALUE`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PLATFORMV_HAS_COMPOSITE_VALUE: u64 = 50003;
/// PlatformV_HAS_MAPPED_VALUE.
///
/// IANA tag: `50004`
/// IANA semantics: `PlatformV_HAS_MAPPED_VALUE`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PLATFORMV_HAS_MAPPED_VALUE: u64 = 50004;
/// PlatformV_HAS_OBJ_ID.
///
/// IANA tag: `50005`
/// IANA semantics: `PlatformV_HAS_OBJ_ID`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PLATFORMV_HAS_OBJ_ID: u64 = 50005;
/// PlatformV_HAS_OBJ_TAG.
///
/// IANA tag: `50006`
/// IANA semantics: `PlatformV_HAS_OBJ_TAG`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PLATFORMV_HAS_OBJ_TAG: u64 = 50006;
/// PlatformV_HAS_CHILD.
///
/// IANA tag: `50007`
/// IANA semantics: `PlatformV_HAS_CHILD`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PLATFORMV_HAS_CHILD: u64 = 50007;
/// PlatformV_HAS_PROPERTY.
///
/// IANA tag: `50008`
/// IANA semantics: `PlatformV_HAS_PROPERTY`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PLATFORMV_HAS_PROPERTY: u64 = 50008;
/// PlatformV_HAS_META.
///
/// IANA tag: `50009`
/// IANA semantics: `PlatformV_HAS_META`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PLATFORMV_HAS_META: u64 = 50009;
/// PlatformV_HAS_EVENT.
///
/// IANA tag: `50010`
/// IANA semantics: `PlatformV_HAS_EVENT`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PLATFORMV_HAS_EVENT: u64 = 50010;
/// PlatformV_HAS_ACTION.
///
/// IANA tag: `50011`
/// IANA semantics: `PlatformV_HAS_ACTION`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PLATFORMV_HAS_ACTION: u64 = 50011;
/// PlatformV_IS_TYPE.
///
/// IANA tag: `50012`
/// IANA semantics: `PlatformV_IS_TYPE`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PLATFORMV_IS_TYPE: u64 = 50012;
/// A tag value of 51997 indicates that the payload is CBOR-LD..
///
/// IANA tag: `51997`
/// IANA semantics: `A tag value of 51997 indicates that the payload is CBOR-LD.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CBOR_LD: u64 = 51997;
/// Self-described CBOR.
///
/// IANA tag: `55799`
/// IANA semantics: `Self-described CBOR; see Section 3.4.6`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const SELF_DESCRIBED_CBOR: u64 = 55799;
/// indicates that the file contains CBOR Sequences.
///
/// IANA tag: `55800`
/// IANA semantics: `indicates that the file contains CBOR Sequences`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CBOR_SEQUENCE_FILE: u64 = 55800;
/// indicates that the file starts with a CBOR-Labeled Non-CBOR Data label..
///
/// IANA tag: `55801`
/// IANA semantics: `indicates that the file starts with a CBOR-Labeled Non-CBOR Data label.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CBOR_LABELED_NON_CBOR_FILE: u64 = 55801;
/// Compressed byte string.
///
/// IANA tag: `56500`
/// IANA semantics: `Compressed byte string`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const COMPRESSED_BYTE_STRING: u64 = 56500;
/// Identify and define a set of record structures (each a sequence of property names) that can be referenced as tags in the included value (and the scope for the record tag definitions).
///
/// IANA tag: `57342`
/// IANA semantics: `Identify and define a set of record structures (each a sequence of property names) that can be referenced as tags in the included value (and the scope for the record tag definitions)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const IDENTIFY_AND_DEFINE_A_SET_OF_RECORD: u64 = 57342;
/// Identify and define a record structure (a sequence of property names), and use that record structure definition to interpret the included values..
///
/// IANA tag: `57343`
/// IANA semantics: `Identify and define a record structure (a sequence of property names), and use that record structure definition to interpret the included values.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const RECORD_STRUCTURE: u64 = 57343;
/// The tagged CBOR array contains attestation evidence data with an Intel TEE quote..
///
/// IANA tag: `60000`
/// IANA semantics: `The tagged CBOR array contains attestation evidence data with an Intel TEE quote.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const THE_TAGGED_CBOR_ARRAY_CONTAINS: u64 = 60000;
/// The tagged CBOR array contains attestation evidence data with an Intel TEE report..
///
/// IANA tag: `60001`
/// IANA semantics: `The tagged CBOR array contains attestation evidence data with an Intel TEE report.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const INTEL_TEE_REPORT: u64 = 60001;
/// The tagged CBOR array contains attestation evidence data with an Intel SGX report..
///
/// IANA tag: `60002`
/// IANA semantics: `The tagged CBOR array contains attestation evidence data with an Intel SGX report.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const INTEL_SGX_REPORT: u64 = 60002;
/// The tagged CBOR array containing a numeric expression..
///
/// IANA tag: `60010`
/// IANA semantics: `The tagged CBOR array containing a numeric expression.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const THE_TAGGED_CBOR_ARRAY_CONTAINING_A: u64 = 60010;
/// The tagged CBOR array containing a set of digests expression..
///
/// IANA tag: `60020`
/// IANA semantics: `The tagged CBOR array containing a set of digests expression.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const DIGESTS_EXPRESSION: u64 = 60020;
/// The tagged CBOR array containing a set of strings expression..
///
/// IANA tag: `60021`
/// IANA semantics: `The tagged CBOR array containing a set of strings expression.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const STRINGS_EXPRESSION: u64 = 60021;
/// always invalid.
///
/// IANA tag: `65535`
/// IANA semantics: `always invalid; see Section 10.1`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ALWAYS_INVALID_LABEL_65535: u64 = 65535;
/// ZeWIF (Zcash Wallet Interchange Format) document; the tag encloses  a two-element array.
///
/// IANA tag: `133133`
/// IANA semantics: `ZeWIF (Zcash Wallet Interchange Format) document; the tag encloses  a two-element array \[version, payload\] whose payload is a CBOR map conforming to the  version's ZeWIF schema. In a stored document this tag is enclosed in tag 55799 (Self-Described CBOR).`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ZEWIF: u64 = 133133;
/// RAINS Message.
///
/// IANA tag: `15309736`
/// IANA semantics: `RAINS Message`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const RAINS_MESSAGE: u64 = 15309736;
/// TCG DICE Protection Environment profile descriptor.
///
/// IANA tag: `1146111423`
/// IANA semantics: `TCG DICE Protection Environment profile descriptor`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const TCG_DICE_PE: u64 = 1146111423;
/// MoaT change-of-status marker.
///
/// IANA tag: `1298360423`
/// IANA semantics: `MoaT change-of-status marker`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const MOAT_CHANGE_OF_STATUS_MARKER: u64 = 1298360423;
/// MoaT end-of-file marker.
///
/// IANA tag: `1298493254`
/// IANA semantics: `MoaT end-of-file marker`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const MOAT_END_OF_FILE_MARKER: u64 = 1298493254;
/// MoaT file identifier / details.
///
/// IANA tag: `1299145044`
/// IANA semantics: `MoaT file identifier / details`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const MOAT_FILE_IDENTIFIER_DETAILS: u64 = 1299145044;
/// A CBOR encoded Openswan configuration file, as stored on disk for unit test cases..
///
/// IANA tag: `1330664270`
/// IANA semantics: `A CBOR encoded Openswan configuration file, as stored on disk for unit test cases.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const OPENSWAN_CONFIG: u64 = 1330664270;
/// A cryptographically anchored data structure used for digital  authorship attestation, capturing the authorship process through entangled  Verifiable Delay Functions (VDFs) and behavioral biometrics..
///
/// IANA tag: `1347571280`
/// IANA semantics: `A cryptographically anchored data structure used for digital  authorship attestation, capturing the authorship process through entangled  Verifiable Delay Functions (VDFs) and behavioral biometrics.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const A_CRYPTOGRAPHICALLY_ANCHORED_DATA: u64 = 1347571280;
/// A cryptographic pointer to a full Proof of Process Evidence  Packet, used for embedding authorship claims in space-constrained contexts  (e.g., metadata, QR codes)..
///
/// IANA tag: `1347571281`
/// IANA semantics: `A cryptographic pointer to a full Proof of Process Evidence  Packet, used for embedding authorship claims in space-constrained contexts  (e.g., metadata, QR codes).`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PROOF_OF_PROCESS_POINTER: u64 = 1347571281;
/// ur:provenance, Provenance Mark.
///
/// IANA tag: `1347571542`
/// IANA semantics: `ur:provenance, Provenance Mark`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const UR_PROVENANCE_PROVENANCE_MARK: u64 = 1347571542;
/// Concise Software Identifier (CoSWID).
///
/// IANA tag: `1398229316`
/// IANA semantics: `Concise Software Identifier (CoSWID)`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const CONCISE_SOFTWARE_IDENTIFIER_COSWID: u64 = 1398229316;
/// Explicitly none..
///
/// IANA tag: `1413829460`
/// IANA semantics: `Explicitly none.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const EXPLICITLY_NONE: u64 = 1413829460;
/// An Attestation Result structure produced by Verifiers  appraising Proof of Process Evidence, conveying verification verdicts,  confidence scores, and forensic assessments per the IETF RATS architecture..
///
/// IANA tag: `1463894560`
/// IANA semantics: `An Attestation Result structure produced by Verifiers  appraising Proof of Process Evidence, conveying verification verdicts,  confidence scores, and forensic assessments per the IETF RATS architecture.`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const AN_ATTESTATION_RESULT_STRUCTURE: u64 = 1463894560;
/// Array of content-addressed blocks and ERIS read capabilities.
///
/// IANA tag: `1701996915`
/// IANA semantics: `Array of content-addressed blocks and ERIS read capabilities`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ERIS_BLOCK_ARRAY: u64 = 1701996915;
/// ERIS-FS image header.
///
/// IANA tag: `1701996916`
/// IANA semantics: `ERIS-FS image header`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ERIS_FS_IMAGE_HEADER: u64 = 1701996916;
/// PromiseGrid message envelope.
///
/// IANA tag: `1735551332`
/// IANA semantics: `PromiseGrid message envelope`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const PROMISEGRID_MESSAGE_ENVELOPE: u64 = 1735551332;
/// always invalid.
///
/// IANA tag: `4294967295`
/// IANA semantics: `always invalid; see Section 10.1`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ALWAYS_INVALID_LABEL_4294967295: u64 = 4294967295;
/// Intel FPGA SPDM Manifest.
///
/// IANA tag: `4294967296`
/// IANA semantics: `Intel FPGA SPDM Manifest`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const INTEL_FPGA_SPDM_MANIFEST: u64 = 4294967296;
/// always invalid; Section 10.1.
///
/// IANA tag: `18446744073709551615`
/// IANA semantics: `always invalid; Section 10.1`
/// IANA source: <https://www.iana.org/assignments/cbor-tags/tags.csv>
pub const ALWAYS_INVALID_U64_MAX: u64 = 18446744073709551615;

/// Returns `true` if `value` is a currently assigned CBOR tag.
#[must_use]
#[allow(clippy::too_many_lines)]
pub const fn is_known(value: u64) -> bool {
    matches!(
        value,
        DATE_TIME_STRING
            | EPOCH_DATE_TIME
            | UNSIGNED_BIGNUM
            | NEGATIVE_BIGNUM
            | DECIMAL_FRACTION
            | BIGFLOAT
            | COSE_ENCRYPT0
            | COSE_MAC0
            | COSE_SIGN1
            | COSE_COUNTERSIGNATURE
            | EXPECTED_BASE64URL
            | EXPECTED_BASE64
            | EXPECTED_BASE16
            | ENCODED_CBOR
            | STRING_REF
            | PERL_OBJECT
            | GENERIC_OBJECT
            | SHARED_VALUE
            | SHARED_VALUE_REF
            | RATIONAL_NUMBER
            | ABSENT_VALUE_IN_A_CBOR_ARRAY
            | URI
            | BASE64URL
            | BASE64
            | REGEX
            | MIME_MESSAGE
            | BINARY_UUID
            | LANGUAGE_TAGGED_STRING
            | IDENTIFIER
            | NDARRAY_ROW_MAJOR
            | HOMOGENEOUS_ARRAY
            | IPLD_CONTENT_IDENTIFIER
            | YANG_BITS_DATATYPE
            | YANG_ENUMERATION_DATATYPE
            | YANG_IDENTITYREF_DATATYPE
            | YANG_INSTANCE_IDENTIFIER_DATATYPE
            | YANG_SID
            | IEEE_MAC_ADDRESS
            | IPV4
            | IPV6
            | SELECTIVE_DISCLOSURE_ARRAY_CLAIM
            | SELECTIVE_DISCLOSURE_REDACTED_CLAIM
            | CWT
            | DECOY_MARKER
            | ENCODED_CBOR_SEQUENCE
            | UINT8_TYPED_ARRAY
            | UINT16_BIG_ENDIAN_TYPED_ARRAY
            | UINT32_BIG_ENDIAN_TYPED_ARRAY
            | UINT64_BIG_ENDIAN_TYPED_ARRAY
            | TYPED_ARRAY_U8_CLAMPED
            | UINT16_LITTLE_ENDIAN_TYPED_ARRAY
            | UINT32_LITTLE_ENDIAN_TYPED_ARRAY
            | UINT64_LITTLE_ENDIAN_TYPED_ARRAY
            | SINT8_TYPED_ARRAY
            | SINT16_BIG_ENDIAN_TYPED_ARRAY
            | SINT32_BIG_ENDIAN_TYPED_ARRAY
            | SINT64_BIG_ENDIAN_TYPED_ARRAY
            | RESERVED
            | SINT16_LITTLE_ENDIAN_TYPED_ARRAY
            | SINT32_LITTLE_ENDIAN_TYPED_ARRAY
            | SINT64_LITTLE_ENDIAN_TYPED_ARRAY
            | TYPED_ARRAY_F16_BE
            | TYPED_ARRAY_F32_BE
            | TYPED_ARRAY_F64_BE
            | TYPED_ARRAY_F128_BE
            | TYPED_ARRAY_F16_LE
            | TYPED_ARRAY_F32_LE
            | TYPED_ARRAY_F64_LE
            | TYPED_ARRAY_F128_LE
            | COSE_ENCRYPTED_DATA_OBJECT
            | COSE_MACED_DATA_OBJECT
            | COSE_SIGNED_DATA_OBJECT
            | CRI_REFERENCE
            | NUMBER_OF_DAYS_SINCE_THE_EPOCH_DATE
            | ALTERNATIVE_BASE128
            | GEOGRAPHIC_COORDINATES
            | GEO_CRS
            | SUIT_ENVELOPE
            | EXPECTED_BASE16_LOWER
            | OID_RELATIVE_BER
            | OID_BER
            | OID_BER_RELATIVE_1_3_6_1
            | INTERNET_OF_THINGS_DATA_POINT
            | GORDIAN_ENVELOPE
            | ENCLOSED_DCBOR
            | STRING_REFS_MARK
            | BINARY_MIME_MESSAGE
            | MATHEMATICAL_FINITE_SET
            | MAP_WITH_KV_OPS
            | NETWORK_ADDRESS_IPV4_OR_IPV6_OR_MAC
            | NETWORK_ADDRESS_PREFIX_IPV4_OR_IPV6
            | EMBEDDED_JSON_OBJECT
            | HEXADECIMAL_STRING
            | DECIMAL_FRACTION_EXT
            | BIGFLOAT_EXT
            | IRI
            | IRI_REF
            | EXTENDED_DECIMAL_FRACTION
            | EXTENDED_BIGFLOAT
            | EXTENDED_RATIONAL_NUMBER
            | DOTS_SIGNAL
            | NON_UTF_8_CESU_8_STRING
            | NON_UTF_8_WTF_8_STRING
            | NON_UTF_8_MUTF_8_STRING
            | TEXT_KEY_MAP
            | ERIS_BINARY_READ_CAPABILITY
            | GAD_SHAPE
            | GAD_VELOCITY
            | COORDINATE_REFERENCE_SYSTEM_WRAPPER
            | SYMBOL
            | LINKED_LIST
            | CHARACTER
            | OBJECT
            | JSON_NUMBER_TEXT
            | SUIT_REPORT_PROTECTED
            | SUIT_REFERENCE
            | SUIT_CAPABILITY_REPORT
            | SHARED_VALUE_SCOPE
            | GEOHASH_STRING
            | EARMARKED_FOR_CORIM
            | CORIM_MAP
            | CONCISE_SWID
            | CONCISE_MID
            | XCORIM_MAP
            | XCORIM_OR_SIGNED_XCORIM
            | UEID
            | EARMARKED_FOR_CORIM_LABEL_551
            | CORIM_SVN
            | CORIM_MIN_SVN
            | SUBJECT_PUBLIC_KEY_INFO_PEM
            | SUBJECT_PUBLIC_KEY_INFO_PEM_B
            | X509_CERT_CHAIN
            | BIT_ARRAY
            | SPDM_TOC_MAP
            | CONCISE_EVIDENCE_MAP
            | UNPROTECTED_CWT_CLAIMS_SET
            | DETACHED_EAT_BUNDLE
            | EXTENDED_TIME
            | DURATION
            | PERIOD
            | TAG_1004
            | OBJECT_TYPE_IDENTIFIER
            | NDARRAY_COLUMN_MAJOR
            | IEEE_OUI_CID
            | SUIT_MANIFEST
            | TAG_18556
            | VALUE_DESCRIPTION_HINT
            | I_REGEXP
            | ECMASCRIPT_REGEXP
            | ALWAYS_INVALID_IN_INTERCHANGE
            | A_CBOR_TAG_IDENTIFIER
            | INDIRECTION_HINT
            | CAPTURE
            | IDENTIFIER_FOR_A_FHIR_CONSTANT
            | EXTERNAL_REFERENCE
            | LOGICAL_NONE
            | LOGICAL_ANY
            | LOGICAL_ALL
            | UR_KNOWN_VALUE_SEMANTIC_SIGNIFIER
            | UR_DIGEST_32_BYTE_SHA_256_DIGEST
            | UR_ENCRYPTED_IETF_CHACHA20_POLY1305
            | UR_COMPRESSED
            | UR_REQUEST_TRANSACTION_REQUEST
            | UR_RESPONSE_TRANSACTION_RESPONSE
            | UR_FUNCTION_ENVELOPE_EXPRESSION
            | UR_PARAMETER_ENVELOPE_EXPRESSION
            | UR_PLACEHOLDER_ENVELOPE_EXPRESSION
            | UR_REPLACEMENT_ENVELOPE_EXPRESSION
            | UR_AGREEMENT_PRIVATE_KEY_CURVE25519
            | UR_AGREEMENT_PUBLIC_KEY_CURVE25519
            | UR_ARID
            | UR_CRYPTO_PRVKEYS_PRIVATE_KEYS_FOR
            | UR_NONCE_CRYPTOGRAPHIC_NONCE
            | UR_PASSWORD_SCRYPT_HASHED_PASSWORD
            | UR_CRYPTO_PRVKEYS_BASE
            | UR_CRYPTO_PUBKEYS_PUBLIC_KEY_BASE
            | UR_SALT
            | UR_CRYPTO_SEALED
            | UR_SIGNATURE
            | UR_SIGNING_PRIVATE_KEY
            | UR_SIGNING_PUBLIC_KEY_CRYPTOGRAPHIC
            | UR_CRYPTO_KEY
            | UR_XID_EXTENSIBLE_IDENTIFIER_OR_XID
            | UR_REFERENCE
            | UR_EVENT_EVENT_IDENTIFIER
            | UR_ENCRYPTED_KEY
            | UR_MLKEM_PRIVATE_KEY
            | UR_MLKEM_PUBLIC_KEY_PUBLIC_KEY_FOR
            | UR_MLKEM_CIPHERTEXT_CIPHERTEXT_FOR
            | UR_MLDSA_PRIVATE_KEY
            | UR_MLDSA_PUBLIC_KEY_PUBLIC_KEY_FOR
            | UR_MLDSA_SIGNATURE_MLDSA_SIGNATURE
            | UR_SEED_CRYPTOGRAPHIC_SEED
            | UR_HDKEY_BITCOIN_BIP_32_HD_KEY
            | UR_KEYPATH_BITCOIN_BIP_32_KEY
            | UR_COIN_INFO
            | UR_ECKEY_BITCOIN_ELLIPTIC_CURVE_KEY
            | UR_ADDRESS_CRYPTOCURRENCY_ADDRESS
            | UR_OUTPUT_DESCRIPTOR_BITCOIN_OUTPUT
            | UR_SSKR_SHARDED_SECRET_KEY
            | UR_PSBT_PARTIALLY_SIGNED_BITCOIN
            | UR_ACCOUNT_DESCRIPTOR
            | UR_SSH_PRIVATE
            | UR_SSH_PUBLIC
            | UR_SSH_SIGNATURE_TEXT_FORMAT_SSH
            | UR_SSH_CERTIFICATE_TEXT_FORMAT_SSH
            | CONCORDIUM_SMART_CONTRACT_ADDRESS
            | FRACTION
            | FRACTION_NAN_SIGNALS
            | FRACTION_NAN_SIGNALS_LABEL_41730
            | FRACTION_BOTH_NANS_SIGNAL
            | A_CONFIDENTIALITY_CLEARANCE_THE_KEY
            | NCMS_METADATA_BINDING
            | NCMS_METADATA_COLLECTION
            | COMPLEX_NUMBER
            | COMPLEX_NUMBER_ARRAY
            | PLATFORMV_IS_ID
            | PLATFORMV_IS_NAME
            | PLATFORMV_IS_VALUE
            | PLATFORMV_HAS_COMPOSITE_VALUE
            | PLATFORMV_HAS_MAPPED_VALUE
            | PLATFORMV_HAS_OBJ_ID
            | PLATFORMV_HAS_OBJ_TAG
            | PLATFORMV_HAS_CHILD
            | PLATFORMV_HAS_PROPERTY
            | PLATFORMV_HAS_META
            | PLATFORMV_HAS_EVENT
            | PLATFORMV_HAS_ACTION
            | PLATFORMV_IS_TYPE
            | CBOR_LD
            | SELF_DESCRIBED_CBOR
            | CBOR_SEQUENCE_FILE
            | CBOR_LABELED_NON_CBOR_FILE
            | COMPRESSED_BYTE_STRING
            | IDENTIFY_AND_DEFINE_A_SET_OF_RECORD
            | RECORD_STRUCTURE
            | THE_TAGGED_CBOR_ARRAY_CONTAINS
            | INTEL_TEE_REPORT
            | INTEL_SGX_REPORT
            | THE_TAGGED_CBOR_ARRAY_CONTAINING_A
            | DIGESTS_EXPRESSION
            | STRINGS_EXPRESSION
            | ALWAYS_INVALID_LABEL_65535
            | ZEWIF
            | RAINS_MESSAGE
            | TCG_DICE_PE
            | MOAT_CHANGE_OF_STATUS_MARKER
            | MOAT_END_OF_FILE_MARKER
            | MOAT_FILE_IDENTIFIER_DETAILS
            | OPENSWAN_CONFIG
            | A_CRYPTOGRAPHICALLY_ANCHORED_DATA
            | PROOF_OF_PROCESS_POINTER
            | UR_PROVENANCE_PROVENANCE_MARK
            | CONCISE_SOFTWARE_IDENTIFIER_COSWID
            | EXPLICITLY_NONE
            | AN_ATTESTATION_RESULT_STRUCTURE
            | ERIS_BLOCK_ARRAY
            | ERIS_FS_IMAGE_HEADER
            | PROMISEGRID_MESSAGE_ENVELOPE
            | ALWAYS_INVALID_LABEL_4294967295
            | INTEL_FPGA_SPDM_MANIFEST
            | ALWAYS_INVALID_U64_MAX
    )
}
