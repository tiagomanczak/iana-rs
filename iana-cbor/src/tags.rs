//! CBOR Tags (IANA registry: cbor-tags/tags.csv).
//!
//! Reference: RFC 8949.

#![allow(clippy::unreadable_literal, clippy::doc_markdown)]

/// CBOR Tag numbers.
/// Standard date/time string; see Section 3.4.1.
pub const TAG_0: u64 = 0;
/// Epoch-based date/time; see Section 3.4.2.
pub const TAG_1: u64 = 1;
/// Unsigned bignum; see Section 3.4.3.
pub const TAG_2: u64 = 2;
/// Negative bignum; see Section 3.4.3.
pub const TAG_3: u64 = 3;
/// Decimal fraction; see Section 3.4.4.
pub const TAG_4: u64 = 4;
/// Bigfloat; see Section 3.4.4.
pub const TAG_5: u64 = 5;
/// COSE Single Recipient Encrypted Data Object.
pub const TAG_16: u64 = 16;
/// COSE Mac w/o Recipients Object.
pub const TAG_17: u64 = 17;
/// COSE Single Signer Data Object.
pub const TAG_18: u64 = 18;
/// COSE standalone V2 countersignature.
pub const TAG_19: u64 = 19;
/// Expected conversion to base64url encoding; see Section 3.4.5.2.
pub const TAG_21: u64 = 21;
/// Expected conversion to base64 encoding; see Section 3.4.5.2.
pub const TAG_22: u64 = 22;
/// Expected conversion to base16 encoding; see Section 3.4.5.2.
pub const TAG_23: u64 = 23;
/// Encoded CBOR data item; see Section 3.4.5.1.
pub const TAG_24: u64 = 24;
/// reference the nth previously seen string.
pub const TAG_25: u64 = 25;
/// Serialised Perl object with classname and constructor arguments.
pub const TAG_26: u64 = 26;
/// Serialised language-independent object with type name and constructor arguments.
pub const TAG_27: u64 = 27;
/// mark value as (potentially) shared.
pub const TAG_28: u64 = 28;
/// reference nth marked value.
pub const TAG_29: u64 = 29;
/// Rational number.
pub const TAG_30: u64 = 30;
/// Absent value in a CBOR Array.
pub const TAG_31: u64 = 31;
/// URI; see Section 3.4.5.3.
pub const TAG_32: u64 = 32;
/// base64url; see Section 3.4.5.3.
pub const TAG_33: u64 = 33;
/// base64; see Section 3.4.5.3.
pub const TAG_34: u64 = 34;
/// Regular expression; see Section 2.4.4.3.
pub const TAG_35: u64 = 35;
/// MIME message; see Section 3.4.5.3.
pub const TAG_36: u64 = 36;
/// Binary UUID (\[RFC9562, Section 4\]).
pub const TAG_37: u64 = 37;
/// Language-tagged string.
pub const TAG_38: u64 = 38;
/// Identifier.
pub const TAG_39: u64 = 39;
/// Multi-dimensional Array, row-major order.
pub const TAG_40: u64 = 40;
/// Homogeneous Array.
pub const TAG_41: u64 = 41;
/// IPLD content identifier.
pub const TAG_42: u64 = 42;
/// YANG bits datatype; see Section 6.7..
pub const TAG_43: u64 = 43;
/// YANG enumeration datatype; see Section 6.6..
pub const TAG_44: u64 = 44;
/// YANG identityref datatype; see Section 6.10..
pub const TAG_45: u64 = 45;
/// YANG instance-identifier datatype; see Section 6.13..
pub const TAG_46: u64 = 46;
/// YANG Schema Item iDentifier (sid); see Section 3.2..
pub const TAG_47: u64 = 47;
/// IEEE MAC Address.
pub const TAG_48: u64 = 48;
/// IPv4, \[prefixlen,IPv4\], \[IPv4,prefixpart\].
pub const TAG_52: u64 = 52;
/// IPv6, \[prefixlen,IPv6\], \[IPv6,prefixpart\].
pub const TAG_54: u64 = 54;
/// An array claim element intended to be redacted, or a map key whose key and value are intended to be redacted. (TEMPORARY - registered 2025-12-09, expires 2026-12-09).
pub const TAG_58: u64 = 58;
/// A selective disclosure redacted (array) claim element. (TEMPORARY - registered 2025-12-09, expires 2026-12-09).
pub const TAG_60: u64 = 60;
/// CBOR Web Token (CWT).
pub const TAG_61: u64 = 61;
/// A marker of a location in a map or an array where a decoy is intended to be inserted. (TEMPORARY - registered 2026-01-27, expires 2027-01-27).
pub const TAG_62: u64 = 62;
/// Encoded CBOR Sequence \[RFC8742\].
pub const TAG_63: u64 = 63;
/// uint8 Typed Array.
pub const TAG_64: u64 = 64;
/// uint16, big endian, Typed Array.
pub const TAG_65: u64 = 65;
/// uint32, big endian, Typed Array.
pub const TAG_66: u64 = 66;
/// uint64, big endian, Typed Array.
pub const TAG_67: u64 = 67;
/// uint8 Typed Array, clamped arithmetic.
pub const TAG_68: u64 = 68;
/// uint16, little endian, Typed Array.
pub const TAG_69: u64 = 69;
/// uint32, little endian, Typed Array.
pub const TAG_70: u64 = 70;
/// uint64, little endian, Typed Array.
pub const TAG_71: u64 = 71;
/// sint8 Typed Array.
pub const TAG_72: u64 = 72;
/// sint16, big endian, Typed Array.
pub const TAG_73: u64 = 73;
/// sint32, big endian, Typed Array.
pub const TAG_74: u64 = 74;
/// sint64, big endian, Typed Array.
pub const TAG_75: u64 = 75;
/// (reserved).
pub const TAG_76: u64 = 76;
/// sint16, little endian, Typed Array.
pub const TAG_77: u64 = 77;
/// sint32, little endian, Typed Array.
pub const TAG_78: u64 = 78;
/// sint64, little endian, Typed Array.
pub const TAG_79: u64 = 79;
/// IEEE 754 binary16, big endian, Typed Array.
pub const TAG_80: u64 = 80;
/// IEEE 754 binary32, big endian, Typed Array.
pub const TAG_81: u64 = 81;
/// IEEE 754 binary64, big endian, Typed Array.
pub const TAG_82: u64 = 82;
/// IEEE 754 binary128, big endian, Typed Array.
pub const TAG_83: u64 = 83;
/// IEEE 754 binary16, little endian, Typed Array.
pub const TAG_84: u64 = 84;
/// IEEE 754 binary32, little endian, Typed Array.
pub const TAG_85: u64 = 85;
/// IEEE 754 binary64, little endian, Typed Array.
pub const TAG_86: u64 = 86;
/// IEEE 754 binary128, little endian, Typed Array.
pub const TAG_87: u64 = 87;
/// COSE Encrypted Data Object.
pub const TAG_96: u64 = 96;
/// COSE MACed Data Object.
pub const TAG_97: u64 = 97;
/// COSE Signed Data Object.
pub const TAG_98: u64 = 98;
/// CRI Reference.
pub const TAG_99: u64 = 99;
/// Number of days since the epoch date 1970-01-01.
pub const TAG_100: u64 = 100;
/// alternatives as given by the uint + 128; see Section 9.1.
pub const TAG_101: u64 = 101;
/// Geographic Coordinates.
pub const TAG_103: u64 = 103;
/// Geographic Coordinate Reference System WKT or EPSG number.
pub const TAG_104: u64 = 104;
/// SUIT_Envelope as defined in Appendix A of \[RFC-ietf-suit-manifest-34\].
pub const SUIT_ENVELOPE: u64 = 107;
/// Expected conversion to base16 encoding (lowercase).
pub const TAG_108: u64 = 108;
/// relative object identifier (BER encoding); SDNV \[RFC6256\] sequence.
pub const TAG_110: u64 = 110;
/// object identifier (BER encoding).
pub const TAG_111: u64 = 111;
/// object identifier (BER encoding), relative to 1.3.6.1.4.1.
pub const TAG_112: u64 = 112;
/// Internet of Things Data Point.
pub const TAG_120: u64 = 120;
/// Gordian Envelope.
pub const TAG_200: u64 = 200;
/// enclosed dCBOR.
pub const TAG_201: u64 = 201;
/// mark value as having string references.
pub const TAG_256: u64 = 256;
/// Binary MIME message.
pub const TAG_257: u64 = 257;
/// Mathematical finite set.
pub const TAG_258: u64 = 258;
/// Map datatype with key-value operations (e.g. `.get()/.set()/.delete()`).
pub const TAG_259: u64 = 259;
/// Network Address (IPv4 or IPv6 or MAC Address) (DEPRECATED in favor of 52 and 54         for IP addresses).
pub const TAG_260: u64 = 260;
/// Network Address Prefix (IPv4 or IPv6 Address + Mask Length) (DEPRECATED in favor of 52 and 54         for IP addresses).
pub const TAG_261: u64 = 261;
/// Embedded JSON Object.
pub const TAG_262: u64 = 262;
/// Hexadecimal string.
pub const TAG_263: u64 = 263;
/// Decimal fraction with arbitrary exponent.
pub const TAG_264: u64 = 264;
/// Bigfloat with arbitrary exponent.
pub const TAG_265: u64 = 265;
/// Internationalized resource identifier (IRI).
pub const TAG_266: u64 = 266;
/// Internationalized resource identifier reference (IRI reference).
pub const TAG_267: u64 = 267;
/// Extended decimal fraction.
pub const TAG_268: u64 = 268;
/// Extended bigfloat.
pub const TAG_269: u64 = 269;
/// Extended rational number.
pub const TAG_270: u64 = 270;
/// DDoS Open Threat Signaling (DOTS) signal channel object, as defined in \[RFC9132\].
pub const TAG_271: u64 = 271;
/// Non-UTF-8 CESU-8 string.
pub const TAG_272: u64 = 272;
/// Non-UTF-8 WTF-8 string.
pub const TAG_273: u64 = 273;
/// Non-UTF-8 MUTF-8 string.
pub const TAG_274: u64 = 274;
/// Map contains only keys that are of type Text String (major type 3).
pub const TAG_275: u64 = 275;
/// ERIS binary read capability.
pub const TAG_276: u64 = 276;
/// Universal Geographical Area Description (GAD) shape; see Section 5.
pub const TAG_277: u64 = 277;
/// Universal Geographical Area Description (GAD) description of velocity; see Section 8.
pub const TAG_278: u64 = 278;
/// Coordinate Reference System Wrapper.
pub const TAG_279: u64 = 279;
/// Symbol.
pub const TAG_280: u64 = 280;
/// Linked list.
pub const TAG_281: u64 = 281;
/// Character.
pub const TAG_282: u64 = 282;
/// Object.
pub const TAG_283: u64 = 283;
/// JSON Numeric Value, Represented as its JSON Text.
pub const TAG_284: u64 = 284;
/// SUIT_Report_Protected.
pub const TAG_285: u64 = 285;
/// SUIT_Reference.
pub const TAG_286: u64 = 286;
/// SUIT_Capability_Report.
pub const TAG_287: u64 = 287;
/// isolate shared values within this scope.
pub const TAG_296: u64 = 296;
/// Geohash String.
pub const TAG_301: u64 = 301;
/// Earmarked for CoRIM.
pub const TAG_500: u64 = 500;
/// A CBOR tag that contains a corim-map..
pub const TAG_501: u64 = 501;
/// A CBOR tag that contains a conciseswid-tag-map..
pub const TAG_505: u64 = 505;
/// A CBOR tag that contains a concisemid-tag-map..
pub const TAG_506: u64 = 506;
/// A CBOR tag that contains an xcorim-map..
pub const TAG_526: u64 = 526;
/// A CBOR tag that contains either: xcorimmap, or signed-xcorim..
pub const TAG_527: u64 = 527;
/// A CBOR tag that contains a UEID between 7 and 33 bytes..
pub const TAG_550: u64 = 550;
/// Earmarked for CoRIM.
pub const TAG_551: u64 = 551;
/// A CBOR tag that contains a security version number that is evaluated with equivalence semantics..
pub const TAG_552: u64 = 552;
/// A CBOR tag that contains min-svn that identifies a security version number that is evaluated with greater than or equals semantics.
pub const TAG_553: u64 = 553;
/// A CBOR tag that contains a PEM encoded SubjectPublicKeyInfo. See Section 13 of \[RFC7468\]..
pub const TAG_554: u64 = 554;
/// A CBOR tag that contains a PEM encoded SubjectPublicKeyInfo. See Section 13 of \[RFC7468\]..
pub const TAG_555: u64 = 555;
/// A CBOR tag that contains an X.509 certificate chain created by the concatenation of as many PEM encoded X.509 certificates as needed. The certificates MUST be concatenated in order that each directly certifies the one preceding..
pub const TAG_556: u64 = 556;
/// A CBOR tag that contains a byte string interpreted as an array of bits..
pub const TAG_560: u64 = 560;
/// spdm-toc-map.
pub const TAG_570: u64 = 570;
/// concise-evidence-map.
pub const TAG_571: u64 = 571;
/// Unprotected CWT Claims Set \[RFC9781\].
pub const TAG_601: u64 = 601;
/// Detached EAT Bundle.
pub const TAG_602: u64 = 602;
/// extended time.
pub const TAG_1001: u64 = 1001;
/// duration.
pub const TAG_1002: u64 = 1002;
/// period.
pub const TAG_1003: u64 = 1003;
/// \[RFC3339\] full-date string.
pub const TAG_1004: u64 = 1004;
/// Object type identifier.
pub const TAG_1010: u64 = 1010;
/// Multi-dimensional Array, column-major order.
pub const TAG_1040: u64 = 1040;
/// IEEE OUI/CID.
pub const TAG_1048: u64 = 1048;
/// SUIT_Manifest as defined in Appendix A of \[RFC-ietf-suit-manifest-34\].
pub const SUIT_MANIFEST: u64 = 1070;
/// \[COSE algorithm identifier, Base Hash value\].
pub const TAG_18556: u64 = 18556;
/// description of the value instead of the value itself.
pub const TAG_20853: u64 = 20853;
/// I-Regexp.
pub const TAG_21065: u64 = 21065;
/// ECMAScript RegExp \[<https://262.ecma-international.org/14.0/#sec-regexp-regular-expression-objects>\].
pub const TAG_21066: u64 = 21066;
/// (always invalid in interchange) programming aid for simple values.
pub const TAG_21334: u64 = 21334;
/// a CBOR Tag identifier.
pub const TAG_21607: u64 = 21607;
/// hint that indicates an additional level of indirection.
pub const TAG_22098: u64 = 22098;
/// Capture \[3\].
pub const TAG_25441: u64 = 25441;
/// Identifier for a FHIR constant.
pub const TAG_32768: u64 = 32768;
/// External reference.
pub const TAG_32769: u64 = 32769;
/// Logical operator: NONE / NOT. Encodes the logical operation (!item1&&!item2&&!item3&&..., if array), otherwise (!item)..
pub const TAG_32870: u64 = 32870;
/// Logical operator: ANY. Encodes the logical operation (item1||item2||item3||...)..
pub const TAG_32871: u64 = 32871;
/// Logical operator: ALL. Encodes the logical operation (item1&&item2&&item3&&...)..
pub const TAG_32872: u64 = 32872;
/// ur:known-value, Semantic signifier.
pub const TAG_40000: u64 = 40000;
/// ur:digest, 32-byte SHA-256 digest.
pub const TAG_40001: u64 = 40001;
/// ur:encrypted, IETF ChaCha20-Poly1305 (\[RFC8439\]) encrypted message.
pub const TAG_40002: u64 = 40002;
/// ur:compressed, \[RFC1951\] DEFLATE-compressed message.
pub const TAG_40003: u64 = 40003;
/// ur:request, Transaction Request identifier.
pub const TAG_40004: u64 = 40004;
/// ur:response, Transaction response identifier.
pub const TAG_40005: u64 = 40005;
/// ur:function, Envelope expression function identifier.
pub const TAG_40006: u64 = 40006;
/// ur:parameter, Envelope expression parameter identifier.
pub const TAG_40007: u64 = 40007;
/// ur:placeholder, Envelope expression placeholder identifier.
pub const TAG_40008: u64 = 40008;
/// ur:replacement, Envelope expression replacement identifier.
pub const TAG_40009: u64 = 40009;
/// ur:agreement-private-key, Curve25519 private key for X25519 key agreement.
pub const TAG_40010: u64 = 40010;
/// ur:agreement-public-key, Curve25519 public key for X25519 key agreement.
pub const TAG_40011: u64 = 40011;
/// ur:arid, Apparently Random Identifier.
pub const TAG_40012: u64 = 40012;
/// ur:crypto-prvkeys, Private keys for cryptographic operations.
pub const TAG_40013: u64 = 40013;
/// ur:nonce, Cryptographic nonce.
pub const TAG_40014: u64 = 40014;
/// ur:password, Scrypt-hashed password.
pub const TAG_40015: u64 = 40015;
/// ur:crypto-prvkeys, Private key base (key material).
pub const TAG_40016: u64 = 40016;
/// ur:crypto-pubkeys, Public key base (signing and agreement public key bundle).
pub const TAG_40017: u64 = 40017;
/// ur:salt, Random salt used for hash tree decorrelation.
pub const TAG_40018: u64 = 40018;
/// ur:crypto-sealed, Encrypted message and ephemeral public key.
pub const TAG_40019: u64 = 40019;
/// ur:signature, Cryptographic signature.
pub const TAG_40020: u64 = 40020;
/// ur:signing-private-key, Cryptographic private key used for signing.
pub const TAG_40021: u64 = 40021;
/// ur:signing-public-key, Cryptographic public key used for signing.
pub const TAG_40022: u64 = 40022;
/// ur:crypto-key, Cryptographic key used for symmetric encryption.
pub const TAG_40023: u64 = 40023;
/// ur:xid, Extensible identifier or XID Document.
pub const TAG_40024: u64 = 40024;
/// ur:reference, Cryptographically secure reference to an object.
pub const TAG_40025: u64 = 40025;
/// ur:event, Event identifier.
pub const TAG_40026: u64 = 40026;
/// ur:encrypted-key, Content key encrypted with a derivation function.
pub const TAG_40027: u64 = 40027;
/// ur:mlkem-private-key, Private key for MLKEM key encapsulation.
pub const TAG_40100: u64 = 40100;
/// ur:mlkem-public-key, Public key for MLKEM key encapsulation.
pub const TAG_40101: u64 = 40101;
/// ur:mlkem-ciphertext, Ciphertext for MLKEM key encapsulation.
pub const TAG_40102: u64 = 40102;
/// ur:mldsa-private-key, Private key for MLDSA signature generation.
pub const TAG_40103: u64 = 40103;
/// ur:mldsa-public-key, Public key for MLDSA signature verification.
pub const TAG_40104: u64 = 40104;
/// ur:mldsa-signature, MLDSA signature.
pub const TAG_40105: u64 = 40105;
/// ur:seed, Cryptographic seed.
pub const TAG_40300: u64 = 40300;
/// ur:hdkey, Bitcoin BIP-32 HD key.
pub const TAG_40303: u64 = 40303;
/// ur:keypath, Bitcoin BIP-32 key derivation path.
pub const TAG_40304: u64 = 40304;
/// ur:coin-info, Cryptographic asset and network specifier.
pub const TAG_40305: u64 = 40305;
/// ur:eckey, Bitcoin elliptic curve key (private or public).
pub const TAG_40306: u64 = 40306;
/// ur:address, Cryptocurrency address.
pub const TAG_40307: u64 = 40307;
/// ur:output-descriptor, Bitcoin output descriptor.
pub const TAG_40308: u64 = 40308;
/// ur:sskr, Sharded Secret Key Reconstruction (SSKR) share.
pub const TAG_40309: u64 = 40309;
/// ur:psbt, Partially Signed Bitcoin Transaction.
pub const TAG_40310: u64 = 40310;
/// ur:account-descriptor, Bitcoin account descriptor.
pub const TAG_40311: u64 = 40311;
/// ur:ssh-private, Text format SSH private key.
pub const TAG_40800: u64 = 40800;
/// ur:ssh-public, Text format SSH public key.
pub const TAG_40801: u64 = 40801;
/// ur:ssh-signature, Text format SSH signature.
pub const TAG_40802: u64 = 40802;
/// ur:ssh-certificate, Text format SSH certificate.
pub const TAG_40803: u64 = 40803;
/// Concordium smart contract address.
pub const TAG_40919: u64 = 40919;
/// Fraction.
pub const TAG_41728: u64 = 41728;
/// Fraction (-NaN signals).
pub const TAG_41729: u64 = 41729;
/// Fraction (+NaN signals).
pub const TAG_41730: u64 = 41730;
/// Fraction (Both NaNs signal).
pub const TAG_41731: u64 = 41731;
/// A confidentiality clearance. The key value pairs of the map are defined in ADatP-4774.8.
pub const TAG_42600: u64 = 42600;
/// A metadata binding. The elements of the array are defined in AdatP-4778.8. The tag is also used as part of the magic number in on-disk detached and encapsulating bindings..
pub const TAG_42601: u64 = 42601;
/// A collection of NCMS metadata elements. The key value pairs of the map are defined in AdatP-5636.8.
pub const TAG_42602: u64 = 42602;
/// Single complex number: array elements are real (I) and imaginary (Q) components.
pub const TAG_43000: u64 = 43000;
/// Array of complex numbers in interleaved form: complex value k is stored with real (I) part  at array index 2k and imaginary (Q) part at index (2k + 1).
pub const TAG_43001: u64 = 43001;
/// PlatformV_IS_ID.
pub const TAG_50000: u64 = 50000;
/// PlatformV_IS_NAME.
pub const TAG_50001: u64 = 50001;
/// PlatformV_IS_VALUE.
pub const TAG_50002: u64 = 50002;
/// PlatformV_HAS_COMPOSITE_VALUE.
pub const TAG_50003: u64 = 50003;
/// PlatformV_HAS_MAPPED_VALUE.
pub const TAG_50004: u64 = 50004;
/// PlatformV_HAS_OBJ_ID.
pub const TAG_50005: u64 = 50005;
/// PlatformV_HAS_OBJ_TAG.
pub const TAG_50006: u64 = 50006;
/// PlatformV_HAS_CHILD.
pub const TAG_50007: u64 = 50007;
/// PlatformV_HAS_PROPERTY.
pub const TAG_50008: u64 = 50008;
/// PlatformV_HAS_META.
pub const TAG_50009: u64 = 50009;
/// PlatformV_HAS_EVENT.
pub const TAG_50010: u64 = 50010;
/// PlatformV_HAS_ACTION.
pub const TAG_50011: u64 = 50011;
/// PlatformV_IS_TYPE.
pub const TAG_50012: u64 = 50012;
/// A tag value of 51997 indicates that the payload is CBOR-LD..
pub const TAG_51997: u64 = 51997;
/// Self-described CBOR; see Section 3.4.6.
pub const TAG_55799: u64 = 55799;
/// indicates that the file contains CBOR Sequences.
pub const TAG_55800: u64 = 55800;
/// indicates that the file starts with a CBOR-Labeled Non-CBOR Data label..
pub const TAG_55801: u64 = 55801;
/// Compressed byte string.
pub const TAG_56500: u64 = 56500;
/// Identify and define a set of record structures (each a sequence of property names) that can be referenced as tags in the included value (and the scope for the record tag definitions).
pub const TAG_57342: u64 = 57342;
/// Identify and define a record structure (a sequence of property names), and use that record structure definition to interpret the included values..
pub const TAG_57343: u64 = 57343;
/// The tagged CBOR array contains attestation evidence data with an Intel TEE quote..
pub const TAG_60000: u64 = 60000;
/// The tagged CBOR array contains attestation evidence data with an Intel TEE report..
pub const TAG_60001: u64 = 60001;
/// The tagged CBOR array contains attestation evidence data with an Intel SGX report..
pub const TAG_60002: u64 = 60002;
/// The tagged CBOR array containing a numeric expression..
pub const TAG_60010: u64 = 60010;
/// The tagged CBOR array containing a set of digests expression..
pub const TAG_60020: u64 = 60020;
/// The tagged CBOR array containing a set of strings expression..
pub const TAG_60021: u64 = 60021;
/// always invalid; see Section 10.1.
pub const TAG_65535: u64 = 65535;
/// ZeWIF (Zcash Wallet Interchange Format) document; the tag encloses  a two-element array \[version, payload\] whose payload is a CBOR map conforming to the  version's ZeWIF schema. In a stored document this tag is enclosed in tag 55799 (Self-Described CBOR)..
pub const TAG_133133: u64 = 133133;
/// RAINS Message.
pub const TAG_15309736: u64 = 15309736;
/// TCG DICE Protection Environment profile descriptor.
pub const TAG_1146111423: u64 = 1146111423;
/// MoaT change-of-status marker.
pub const TAG_1298360423: u64 = 1298360423;
/// MoaT end-of-file marker.
pub const TAG_1298493254: u64 = 1298493254;
/// MoaT file identifier / details.
pub const TAG_1299145044: u64 = 1299145044;
/// A CBOR encoded Openswan configuration file, as stored on disk for unit test cases..
pub const TAG_1330664270: u64 = 1330664270;
/// A cryptographically anchored data structure used for digital  authorship attestation, capturing the authorship process through entangled  Verifiable Delay Functions (VDFs) and behavioral biometrics..
pub const TAG_1347571280: u64 = 1347571280;
/// A cryptographic pointer to a full Proof of Process Evidence  Packet, used for embedding authorship claims in space-constrained contexts  (e.g., metadata, QR codes)..
pub const TAG_1347571281: u64 = 1347571281;
/// ur:provenance, Provenance Mark.
pub const TAG_1347571542: u64 = 1347571542;
/// Concise Software Identifier (CoSWID).
pub const TAG_1398229316: u64 = 1398229316;
/// Explicitly none..
pub const TAG_1413829460: u64 = 1413829460;
/// An Attestation Result structure produced by Verifiers  appraising Proof of Process Evidence, conveying verification verdicts,  confidence scores, and forensic assessments per the IETF RATS architecture..
pub const TAG_1463894560: u64 = 1463894560;
/// Array of content-addressed blocks and ERIS read capabilities.
pub const TAG_1701996915: u64 = 1701996915;
/// ERIS-FS image header.
pub const TAG_1701996916: u64 = 1701996916;
/// PromiseGrid message envelope.
pub const TAG_1735551332: u64 = 1735551332;
/// always invalid; see Section 10.1.
pub const TAG_4294967295: u64 = 4294967295;
/// Intel FPGA SPDM Manifest.
pub const TAG_4294967296: u64 = 4294967296;
/// always invalid; Section 10.1.
pub const TAG_18446744073709551615: u64 = 18446744073709551615;

/// Returns `true` if `value` is a currently assigned CBOR tag.
#[must_use]
#[allow(clippy::too_many_lines)]
pub const fn is_known(value: u64) -> bool {
    matches!(
        value,
        TAG_0
            | TAG_1
            | TAG_2
            | TAG_3
            | TAG_4
            | TAG_5
            | TAG_16
            | TAG_17
            | TAG_18
            | TAG_19
            | TAG_21
            | TAG_22
            | TAG_23
            | TAG_24
            | TAG_25
            | TAG_26
            | TAG_27
            | TAG_28
            | TAG_29
            | TAG_30
            | TAG_31
            | TAG_32
            | TAG_33
            | TAG_34
            | TAG_35
            | TAG_36
            | TAG_37
            | TAG_38
            | TAG_39
            | TAG_40
            | TAG_41
            | TAG_42
            | TAG_43
            | TAG_44
            | TAG_45
            | TAG_46
            | TAG_47
            | TAG_48
            | TAG_52
            | TAG_54
            | TAG_58
            | TAG_60
            | TAG_61
            | TAG_62
            | TAG_63
            | TAG_64
            | TAG_65
            | TAG_66
            | TAG_67
            | TAG_68
            | TAG_69
            | TAG_70
            | TAG_71
            | TAG_72
            | TAG_73
            | TAG_74
            | TAG_75
            | TAG_76
            | TAG_77
            | TAG_78
            | TAG_79
            | TAG_80
            | TAG_81
            | TAG_82
            | TAG_83
            | TAG_84
            | TAG_85
            | TAG_86
            | TAG_87
            | TAG_96
            | TAG_97
            | TAG_98
            | TAG_99
            | TAG_100
            | TAG_101
            | TAG_103
            | TAG_104
            | SUIT_ENVELOPE
            | TAG_108
            | TAG_110
            | TAG_111
            | TAG_112
            | TAG_120
            | TAG_200
            | TAG_201
            | TAG_256
            | TAG_257
            | TAG_258
            | TAG_259
            | TAG_260
            | TAG_261
            | TAG_262
            | TAG_263
            | TAG_264
            | TAG_265
            | TAG_266
            | TAG_267
            | TAG_268
            | TAG_269
            | TAG_270
            | TAG_271
            | TAG_272
            | TAG_273
            | TAG_274
            | TAG_275
            | TAG_276
            | TAG_277
            | TAG_278
            | TAG_279
            | TAG_280
            | TAG_281
            | TAG_282
            | TAG_283
            | TAG_284
            | TAG_285
            | TAG_286
            | TAG_287
            | TAG_296
            | TAG_301
            | TAG_500
            | TAG_501
            | TAG_505
            | TAG_506
            | TAG_526
            | TAG_527
            | TAG_550
            | TAG_551
            | TAG_552
            | TAG_553
            | TAG_554
            | TAG_555
            | TAG_556
            | TAG_560
            | TAG_570
            | TAG_571
            | TAG_601
            | TAG_602
            | TAG_1001
            | TAG_1002
            | TAG_1003
            | TAG_1004
            | TAG_1010
            | TAG_1040
            | TAG_1048
            | SUIT_MANIFEST
            | TAG_18556
            | TAG_20853
            | TAG_21065
            | TAG_21066
            | TAG_21334
            | TAG_21607
            | TAG_22098
            | TAG_25441
            | TAG_32768
            | TAG_32769
            | TAG_32870
            | TAG_32871
            | TAG_32872
            | TAG_40000
            | TAG_40001
            | TAG_40002
            | TAG_40003
            | TAG_40004
            | TAG_40005
            | TAG_40006
            | TAG_40007
            | TAG_40008
            | TAG_40009
            | TAG_40010
            | TAG_40011
            | TAG_40012
            | TAG_40013
            | TAG_40014
            | TAG_40015
            | TAG_40016
            | TAG_40017
            | TAG_40018
            | TAG_40019
            | TAG_40020
            | TAG_40021
            | TAG_40022
            | TAG_40023
            | TAG_40024
            | TAG_40025
            | TAG_40026
            | TAG_40027
            | TAG_40100
            | TAG_40101
            | TAG_40102
            | TAG_40103
            | TAG_40104
            | TAG_40105
            | TAG_40300
            | TAG_40303
            | TAG_40304
            | TAG_40305
            | TAG_40306
            | TAG_40307
            | TAG_40308
            | TAG_40309
            | TAG_40310
            | TAG_40311
            | TAG_40800
            | TAG_40801
            | TAG_40802
            | TAG_40803
            | TAG_40919
            | TAG_41728
            | TAG_41729
            | TAG_41730
            | TAG_41731
            | TAG_42600
            | TAG_42601
            | TAG_42602
            | TAG_43000
            | TAG_43001
            | TAG_50000
            | TAG_50001
            | TAG_50002
            | TAG_50003
            | TAG_50004
            | TAG_50005
            | TAG_50006
            | TAG_50007
            | TAG_50008
            | TAG_50009
            | TAG_50010
            | TAG_50011
            | TAG_50012
            | TAG_51997
            | TAG_55799
            | TAG_55800
            | TAG_55801
            | TAG_56500
            | TAG_57342
            | TAG_57343
            | TAG_60000
            | TAG_60001
            | TAG_60002
            | TAG_60010
            | TAG_60020
            | TAG_60021
            | TAG_65535
            | TAG_133133
            | TAG_15309736
            | TAG_1146111423
            | TAG_1298360423
            | TAG_1298493254
            | TAG_1299145044
            | TAG_1330664270
            | TAG_1347571280
            | TAG_1347571281
            | TAG_1347571542
            | TAG_1398229316
            | TAG_1413829460
            | TAG_1463894560
            | TAG_1701996915
            | TAG_1701996916
            | TAG_1735551332
            | TAG_4294967295
            | TAG_4294967296
            | TAG_18446744073709551615
    )
}
