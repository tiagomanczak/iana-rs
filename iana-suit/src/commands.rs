//! SUIT Commands (IANA registry: suit-commands).
//!
//! Reference: [RFC-ietf-suit-manifest-34](https://www.iana.org/go/draft-ietf-suit-manifest-34)


/// Vendor Identifier.
const VENDOR_IDENTIFIER: i32 = 1;
/// Class Identifier.
const CLASS_IDENTIFIER: i32 = 2;
/// Image Match.
const IMAGE_MATCH: i32 = 3;
/// Component Slot.
const COMPONENT_SLOT: i32 = 5;
/// Check Content.
const CHECK_CONTENT: i32 = 6;
/// Dependency Integrity.
const DEPENDENCY_INTEGRITY: i32 = 7;
/// Is Dependency.
const IS_DEPENDENCY: i32 = 8;
/// Process Dependency.
const PROCESS_DEPENDENCY: i32 = 11;
/// Set Component Index.
const SET_COMPONENT_INDEX: i32 = 12;
/// Abort.
const ABORT: i32 = 14;
/// Try Each.
const TRY_EACH: i32 = 15;
/// Write Content.
const WRITE_CONTENT: i32 = 18;
/// Set Parameters.
const SET_PARAMETERS: i32 = 19;
/// Override Parameters.
const OVERRIDE_PARAMETERS: i32 = 20;
/// Fetch.
const FETCH: i32 = 21;
/// Copy.
const COPY: i32 = 22;
/// Invoke.
const INVOKE: i32 = 23;
/// Device Identifier.
const DEVICE_IDENTIFIER: i32 = 24;
/// Swap.
const SWAP: i32 = 31;
/// Run Sequence.
const RUN_SEQUENCE: i32 = 32;
/// Unlink.
const UNLINK: i32 = 33;

/// A SUIT Command label.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Command(i32);

impl Command {
    /// Vendor Identifier.
    pub const VENDOR_IDENTIFIER: Self = Self(VENDOR_IDENTIFIER);
    /// Class Identifier.
    pub const CLASS_IDENTIFIER: Self = Self(CLASS_IDENTIFIER);
    /// Image Match.
    pub const IMAGE_MATCH: Self = Self(IMAGE_MATCH);
    /// Component Slot.
    pub const COMPONENT_SLOT: Self = Self(COMPONENT_SLOT);
    /// Check Content.
    pub const CHECK_CONTENT: Self = Self(CHECK_CONTENT);
    /// Dependency Integrity.
    pub const DEPENDENCY_INTEGRITY: Self = Self(DEPENDENCY_INTEGRITY);
    /// Is Dependency.
    pub const IS_DEPENDENCY: Self = Self(IS_DEPENDENCY);
    /// Process Dependency.
    pub const PROCESS_DEPENDENCY: Self = Self(PROCESS_DEPENDENCY);
    /// Set Component Index.
    pub const SET_COMPONENT_INDEX: Self = Self(SET_COMPONENT_INDEX);
    /// Abort.
    pub const ABORT: Self = Self(ABORT);
    /// Try Each.
    pub const TRY_EACH: Self = Self(TRY_EACH);
    /// Write Content.
    pub const WRITE_CONTENT: Self = Self(WRITE_CONTENT);
    /// Set Parameters.
    pub const SET_PARAMETERS: Self = Self(SET_PARAMETERS);
    /// Override Parameters.
    pub const OVERRIDE_PARAMETERS: Self = Self(OVERRIDE_PARAMETERS);
    /// Fetch.
    pub const FETCH: Self = Self(FETCH);
    /// Copy.
    pub const COPY: Self = Self(COPY);
    /// Invoke.
    pub const INVOKE: Self = Self(INVOKE);
    /// Device Identifier.
    pub const DEVICE_IDENTIFIER: Self = Self(DEVICE_IDENTIFIER);
    /// Swap.
    pub const SWAP: Self = Self(SWAP);
    /// Run Sequence.
    pub const RUN_SEQUENCE: Self = Self(RUN_SEQUENCE);
    /// Unlink.
    pub const UNLINK: Self = Self(UNLINK);

    /// Returns the raw numeric label.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.0
    }
}

impl From<Command> for i32 {
    fn from(value: Command) -> Self {
        value.0
    }
}

impl TryFrom<i32> for Command {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if is_known(value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

/// Returns `true` if `label` is a currently assigned SUIT Command label.
#[must_use]
pub const fn is_known(label: i32) -> bool {
    matches!(
        label,
        VENDOR_IDENTIFIER
            | CLASS_IDENTIFIER
            | IMAGE_MATCH
            | COMPONENT_SLOT
            | CHECK_CONTENT
            | DEPENDENCY_INTEGRITY
            | IS_DEPENDENCY
            | PROCESS_DEPENDENCY
            | SET_COMPONENT_INDEX
            | ABORT
            | TRY_EACH
            | WRITE_CONTENT
            | SET_PARAMETERS
            | OVERRIDE_PARAMETERS
            | FETCH
            | COPY
            | INVOKE
            | DEVICE_IDENTIFIER
            | SWAP
            | RUN_SEQUENCE
            | UNLINK
    )
}
