# iana-suit

IANA SUIT (Software Update for the Internet of Things) registry constants for Rust.

Every assigned numeric label from the [IANA SUIT registries][iana-suit] is exposed as
a typed, documented `const`. No dependencies, fully `no_std` compatible.

[![Crates.io](https://img.shields.io/crates/v/iana-suit)](https://crates.io/crates/iana-suit)
[![docs.rs](https://img.shields.io/docsrs/iana-suit)](https://docs.rs/iana-suit)

## Usage

```toml
[dependencies]
iana-suit = "0.1"
```

```rust
use iana_suit::commands::FETCH;
use iana_suit::parameters::IMAGE_DIGEST;
use iana_suit::envelope::MANIFEST;

// Match CBOR map keys received off the wire
match key {
    iana_suit::commands::FETCH   => { /* handle fetch */ }
    iana_suit::commands::INVOKE  => { /* handle invoke */ }
    _ => { /* unknown command */ }
}

// Validate before matching
if !iana_suit::commands::is_known(key) {
    return Err(Error::UnknownCommand(key));
}
```

## Registries

All labels are `i32`, matching the signed integer range used by SUIT CBOR maps.
IANA snapshot: **2026-02-17**.

| Module              | IANA Registry                          | Labels |
|---------------------|----------------------------------------|--------|
| `envelope`          | SUIT Envelope Elements                 | 9      |
| `manifest`          | SUIT Manifest Elements                 | 16     |
| `common`            | SUIT Common Elements                   | 4      |
| `commands`          | SUIT Commands                          | 22     |
| `parameters`        | SUIT Parameters                        | 14     |
| `text`              | SUIT Text Values                       | 5      |
| `component_text`    | SUIT Component Text Values             | 7      |
| `report`            | SUIT Report Elements                   | 8      |
| `record`            | SUIT Record Elements                   | 5      |
| `report_reasons`    | SUIT Report Reasons                    | 13     |
| `capability_report` | SUIT Capability Report Elements        | 10     |

Each module also exposes `is_known(label: i32) -> bool`, a `const fn` that returns
`true` if the integer is a currently assigned label in that registry.

## Versioning

Cargo semver is the compatibility contract:

- **Patch** — documentation fixes only
- **Minor** — new constants added (additive, backward compatible)
- **Major** — constants renamed, removed, or retyped

The IANA snapshot date is available as `iana_suit::IANA_SNAPSHOT`.

## References

- [IANA SUIT Registries][iana-suit]
- RFC-ietf-suit-manifest-34
- RFC-ietf-suit-trust-domains-12
- RFC-ietf-suit-report-19

[iana-suit]: https://www.iana.org/assignments/suit/suit.xhtml

## License

Licensed under either of [Apache License, Version 2.0](../LICENSE-APACHE) or
[MIT License](../LICENSE-MIT) at your option.
