# iana-rs

Rust definitions for IANA registries — no dependencies, `no_std` compatible.

IANA registries assign stable numeric labels to well-known protocol concepts.
Without a shared definition crate, teams copy integers by hand and lose the
link back to the registry that defines them. `iana-rs` turns each registry
into a versioned Rust interface: every assigned value is a typed, documented
`const`, and Cargo semver stands in for the registry's own compatibility
guarantees.

## Crates

| Crate | Description | Crates.io |
|-------|-------------|-----------|
| [`iana-suit`](iana-suit/) | SUIT Software Update registries | [![](https://img.shields.io/crates/v/iana-suit)](https://crates.io/crates/iana-suit) |
| [`iana-cbor`](iana-cbor/) | CBOR Tags, Simple Values, Timescales | [![](https://img.shields.io/crates/v/iana-cbor)](https://crates.io/crates/iana-cbor) |
| [`iana-rats`](iana-rats/) | RATS attestation registries (skeleton) | [![](https://img.shields.io/crates/v/iana-rats)](https://crates.io/crates/iana-rats) |

## Quick start

```toml
[dependencies]
iana-suit = "0.1"
iana-cbor = "0.1"
```

```rust
use iana_suit::commands;
use iana_cbor::tags;

// Constants are at module level — no sub-namespace needed.
assert_eq!(commands::FETCH, 21);
assert_eq!(tags::SUIT_ENVELOPE, 107);
assert_eq!(tags::SUIT_MANIFEST, 1070);

// Validate before processing
if !commands::is_known(key) {
    return Err(Error::UnknownCommand(key));
}
```

## `iana-suit`

11 SUIT registry modules, fully populated from IANA snapshot 2026-02-17.
Labels are `i32` (signed, matching SUIT CBOR map keys).

Modules: `envelope`, `manifest`, `common`, `commands`, `parameters`, `text`,
`component_text`, `report`, `record`, `report_reasons`, `capability_report`.

See [`iana-suit/README.md`](iana-suit/README.md) for the full module table.

## `iana-cbor`

4 CBOR registry modules, populated from IANA snapshot 2026-07-20.

| Module              | Type   | Notes                        |
|---------------------|--------|------------------------------|
| `tags`              | `u64`  | 262 assigned tags            |
| `simple_values`     | `u64`  | FALSE, TRUE, NULL, UNDEFINED |
| `timescales`        | `u64`  | UTC, TAI                     |
| `time_tag_map_keys` | `i128` | Values outside `i64` range   |

See [`iana-cbor/README.md`](iana-cbor/README.md) for details.

## `iana-rats`

Skeleton crate. Module structure is in place; constants will be populated
once the IANA RATS CSV files are available.
See [`iana-rats/README.md`](iana-rats/README.md).

## `iana-sync` (development tool)

A Rust-native tool for keeping registry constants in sync with the live IANA
CSV files. Not published to crates.io.

```bash
cargo run -p iana-sync -- check    # verify all registries match IANA
cargo run -p iana-sync -- update   # append missing constants
```

`check` runs on every PR via GitHub Actions. `update` runs on a weekly
schedule and opens a pull request for human review.

## Versioning

Cargo semver is the compatibility contract for all published crates:

- **Patch** — documentation only
- **Minor** — new constants (backward compatible)
- **Major** — renamed, removed, or retyped constants

The IANA snapshot date for each crate is available as `iana_suit::IANA_SNAPSHOT`,
`iana_cbor::IANA_SNAPSHOT`, etc.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.
