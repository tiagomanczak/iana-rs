# iana-cbor

IANA CBOR (Concise Binary Object Representation) registry constants for Rust.

Every assigned value from the [IANA CBOR registries][iana-cbor] is exposed as a
typed, documented `const`. No dependencies, fully `no_std` compatible.

[![Crates.io](https://img.shields.io/crates/v/iana-cbor)](https://crates.io/crates/iana-cbor)
[![docs.rs](https://img.shields.io/docsrs/iana-cbor)](https://docs.rs/iana-cbor)

## Usage

```toml
[dependencies]
iana-cbor = "0.1"
```

```rust
use iana_cbor::tags::SUIT_ENVELOPE;
use iana_cbor::tags::SUIT_MANIFEST;
use iana_cbor::simple_values::FALSE;

assert_eq!(SUIT_ENVELOPE, 107);
assert_eq!(SUIT_MANIFEST, 1070);
assert_eq!(FALSE, 20);

// Validate a tag received off the wire
if !iana_cbor::tags::is_known(tag) {
    return Err(Error::UnknownTag(tag));
}
```

## Registries

IANA snapshot: **2026-07-20**.

| Module              | IANA Registry              | Type   | Labels |
|---------------------|----------------------------|--------|--------|
| `tags`              | CBOR Tags                  | `u64`  | 262    |
| `simple_values`     | CBOR Simple Values         | `u64`  | 5      |
| `timescales`        | CBOR Timescales            | `u64`  | 2      |
| `time_tag_map_keys` | CBOR Time Tag Map Keys     | `i128` | 21     |

**Type rationale:**
- `u64` — CBOR tags and simple values are nonnegative by definition.
- `i128` — The Time Tag Map Keys registry contains values outside the `i64` range
  (e.g. `−18446744073709551616`), requiring `i128` for lossless representation.

Each module exposes `is_known(label) -> bool`, a `const fn` for membership checks.

## Naming conventions for CBOR tags

Tag constant names follow these rules, in order of priority:

1. **Override map wins.** Hand-curated names in `tools/iana-sync` take precedence over
auto-generation. Contribute new names there, not by editing `tags.rs` directly.
2. **Proper nouns and acronyms preserved.** `COSE`, `URI`, `IRI`, `IEEE`, `YANG`,
`CWT`, `UUID`, `OID`, `MIME`, `JSON`, `IPLD` are kept as-is.
3. **Strip IANA prose wrappers.** "A CBOR tag that contains a …-map" → keep only the
payload type name (e.g. `CORIM_MAP`).
4. **Typed Array tags use the pattern `TYPED_ARRAY_{type}_{endian}`.** Types: `U8`,
`U16`, `U32`, `U64`, `F16`, `F32`, `F64`, `F128`. Endian: `BE`, `LE`.
5. **`ur:*` tags use `UR_` prefix** with the `ur:name` component (e.g. `UR_SALT`).
6. **Strip "see Section X.Y"** and similar trailing prose from descriptions.
7. **Max 40 characters**, truncated at the last word boundary.
8. **`TAG_N` is the last resort** — only when no meaningful name can be derived.
9. **Sentinel tags** (`always invalid`) are named `ALWAYS_INVALID_LABEL_{N}` or
`ALWAYS_INVALID_U64_MAX`.
10. **Temporary registrations** keep their name; the doc comment notes the expiry.

To propose a new name: add an entry to `TAG_NAME_OVERRIDES` in
`tools/iana-sync/src/main.rs` and run `cargo run -p iana-sync -- update`.

## SUIT-relevant constants

`iana-cbor` is a direct dependency of SUIT manifest parsers and encoders:

| Constant                        | Value |
|---------------------------------|-------|
| `tags::SUIT_ENVELOPE`           | 107   |
| `tags::SUIT_MANIFEST`           | 1070  |

## Versioning

Cargo semver is the compatibility contract:

- **Patch** — documentation fixes only
- **Minor** — new constants added
- **Major** — constants renamed, removed, or retyped

The IANA snapshot date is available as `iana_cbor::IANA_SNAPSHOT`.

## References

- [IANA CBOR Registries][iana-cbor]
- [RFC 8949 — CBOR](https://www.rfc-editor.org/rfc/rfc8949)
- [RFC 9581 — CBOR Tags for Time](https://www.rfc-editor.org/rfc/rfc9581)

[iana-cbor]: https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml

## License

Licensed under either of [Apache License, Version 2.0](../LICENSE-APACHE) or
[MIT License](../LICENSE-MIT) at your option.
