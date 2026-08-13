# iana-rats

IANA RATS (Remote Attestation Procedures) registry constants for Rust.

> **Status: skeleton.** Module structure is established and ready for constants.
> Labels will be added from the official IANA CSV sources in a subsequent release.
> Do not depend on this crate in production until constants are populated.

[![Crates.io](https://img.shields.io/crates/v/iana-rats)](https://crates.io/crates/iana-rats)
[![docs.rs](https://img.shields.io/docsrs/iana-rats)](https://docs.rs/iana-rats)

## Usage

```toml
[dependencies]
iana-rats = "0.1"
```

## Registries

IANA snapshot: **2026-07-20**.

| Module            | IANA Registry                                   | Status  |
|-------------------|-------------------------------------------------|---------|
| `eat_intended_uses` | EAT Intended Uses                             | Pending |
| `cmw_indicators`    | RATS CMW Indicators                           | Pending |

## Versioning

Cargo semver is the compatibility contract. The IANA snapshot date is available
as `iana_rats::IANA_SNAPSHOT`.

## References

- [IANA RATS Registries](https://www.iana.org/assignments/rats/rats.xhtml)
- [RFC 9711 — EAT](https://www.rfc-editor.org/rfc/rfc9711)

## License

Licensed under either of [Apache License, Version 2.0](../LICENSE-APACHE) or
[MIT License](../LICENSE-MIT) at your option.
