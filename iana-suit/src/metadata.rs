//! Package and source metadata for diagnostics.
//!
//! The Cargo package version is the compatibility contract for this crate.
//! The IANA snapshot is informational only and must not be used for version
//! or compatibility checks.

/// Version and snapshot metadata.
pub mod version {
    /// The package name supplied by Cargo.
    pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

    /// The Cargo package version and compatibility contract.
    pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

    /// Short alias for [`PACKAGE_VERSION`].
    pub const VERSION: &str = PACKAGE_VERSION;

    /// The date of the IANA registry snapshot represented by this crate.
    ///
    /// This is diagnostic metadata; Cargo package semver remains the
    /// compatibility contract.
    pub const IANA_SNAPSHOT: &str = "2026-02-17";

    /// Alias for [`IANA_SNAPSHOT`] with an explicit registry qualifier.
    pub const IANA_REGISTRY_SNAPSHOT: &str = IANA_SNAPSHOT;
}

pub use version::{IANA_REGISTRY_SNAPSHOT, IANA_SNAPSHOT, PACKAGE_NAME, PACKAGE_VERSION, VERSION};
