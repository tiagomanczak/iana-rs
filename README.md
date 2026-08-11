# iana-rs

Rust definitions for IANA registries, starting with complete coverage of the
current SUIT registries.

## `iana-suit`

The crate is dependency-free and `no_std` compatible. Registry labels are
available as `i64` constants:

```rust
use iana_suit::commands::label::FETCH;

assert_eq!(FETCH, 21);
```

The current SUIT modules are:

- Envelope Elements
- Manifest Elements
- Common Elements
- Commands
- Parameters
- Text Values
- Component Text Values
- Report Elements
- Record Elements
- Report Reasons
- Capability Report Elements

Cargo package semver is the compatibility contract. Additive constants are
minor releases; removed, renamed, or changed constants require a major
release. The informational snapshot date is available through
`iana_suit::metadata::version::IANA_SNAPSHOT`.

## `iana-sync`

The workspace includes a Rust-only development tool for synchronizing SUIT
definitions with the official IANA CSV registries:

```bash
cargo run -p iana-sync -- check
cargo run -p iana-sync -- update
```

`check` downloads the current CSV files and verifies assigned numeric labels.
`update` conservatively appends missing constants using names derived from the
IANA descriptions; it never removes stale constants automatically. Review
renames, removals, and Cargo semver changes manually.

The `.github/workflows/iana-sync.yml` workflow runs this process weekly and on
manual dispatch. Changes are opened as a pull request, while pull requests
run the checker and workspace tests in read-only validation mode.

## `iana-rats`

`iana-rats` is the initial skeleton for the IANA Remote Attestation
Procedures registries. It currently contains modules for EAT Intended Uses
and RATS Conceptual Message Wrapper Indicators; their constants will be added
from the official IANA CSV files in a subsequent change.
