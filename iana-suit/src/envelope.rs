//! SUIT Envelope Elements (IANA registry: suit-envelope-elements).
//!
//! Reference: [RFC-ietf-suit-manifest-34](https://www.iana.org/go/draft-ietf-suit-manifest-34)


/// Reserved (Delegation).
const DELEGATION: i32 = 1;
/// Authentication Wrapper.
const AUTHENTICATION_WRAPPER: i32 = 2;
/// Manifest.
const MANIFEST: i32 = 3;
/// Dependency Resolution.
const DEPENDENCY_RESOLUTION: i32 = 15;
/// Payload Fetch.
const PAYLOAD_FETCH: i32 = 16;
/// Candidate Verification.
const CANDIDATE_VERIFICATION: i32 = 18;
/// Payload Installation.
const PAYLOAD_INSTALLATION: i32 = 20;
/// Text Description.
const TEXT_DESCRIPTION: i32 = 23;

/// A SUIT Envelope Element label.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EnvelopeElement(i32);

impl EnvelopeElement {
    /// Reserved (Delegation).
    pub const DELEGATION: Self = Self(DELEGATION);
    /// Authentication Wrapper.
    pub const AUTHENTICATION_WRAPPER: Self = Self(AUTHENTICATION_WRAPPER);
    /// Manifest.
    pub const MANIFEST: Self = Self(MANIFEST);
    /// Dependency Resolution.
    pub const DEPENDENCY_RESOLUTION: Self = Self(DEPENDENCY_RESOLUTION);
    /// Payload Fetch.
    pub const PAYLOAD_FETCH: Self = Self(PAYLOAD_FETCH);
    /// Candidate Verification.
    pub const CANDIDATE_VERIFICATION: Self = Self(CANDIDATE_VERIFICATION);
    /// Payload Installation.
    pub const PAYLOAD_INSTALLATION: Self = Self(PAYLOAD_INSTALLATION);
    /// Text Description.
    pub const TEXT_DESCRIPTION: Self = Self(TEXT_DESCRIPTION);

    /// Returns the raw numeric label.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.0
    }
}

impl From<EnvelopeElement> for i32 {
    fn from(value: EnvelopeElement) -> Self {
        value.0
    }
}

impl TryFrom<i32> for EnvelopeElement {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if is_known(value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

/// Returns `true` if `label` is a currently assigned SUIT Envelope Element label.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        DELEGATION
            | AUTHENTICATION_WRAPPER
            | MANIFEST
            | DEPENDENCY_RESOLUTION
            | PAYLOAD_FETCH
            | CANDIDATE_VERIFICATION
            | PAYLOAD_INSTALLATION
            | TEXT_DESCRIPTION
    )
}

/// SUIT Envelope CBOR tag values.
///
/// | Tag | Name          |
/// |-----|---------------|
/// | 107 | SUIT Envelope |
pub mod tag {
    /// SUIT Envelope.
    const SUIT_ENVELOPE: i32 = 107;

    /// A SUIT Envelope CBOR tag.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Tag(i32);

    impl Tag {
        /// SUIT Envelope.
        pub const SUIT_ENVELOPE: Self = Self(SUIT_ENVELOPE);

        /// Returns the raw numeric tag.
        #[must_use]
        pub const fn as_i32(self) -> i32 {
            self.0
        }
    }

    impl From<Tag> for i32 {
        fn from(value: Tag) -> Self {
            value.0
        }
    }

    impl TryFrom<i32> for Tag {
        type Error = i32;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if is_known(value) {
                Ok(Self(value))
            } else {
                Err(value)
            }
        }
    }

    /// Returns `true` if `label` is a currently assigned SUIT Envelope tag.
    #[must_use]
    pub const fn is_known(label: i32) -> bool {
        matches!(label, SUIT_ENVELOPE)
    }
}
