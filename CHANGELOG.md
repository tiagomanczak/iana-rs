# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

See [ROADMAP.md](ROADMAP.md) for planned future milestones.

## [Unreleased]

### Added

- `ROADMAP.md` at workspace root documenting all planned milestones through v1.0.0
- `iana-cbor/tag_names.toml` — hand-curated CBOR tag name overrides (119 entries); `iana-sync` reads this file at runtime instead of a hard-coded static array

### Changed
- `iana-cbor`: removed `tag_names.toml` — tag constant names now have a single source of truth in `src/tags.rs`; `iana-sync` no longer requires a separate override file
- `tools/iana-sync`: removed `load_tag_name_overrides()`, `TagNamesToml`, and the `overrides` parameter chain; new IANA tags get auto-generated names, developer renames manually in `tags.rs`
- `tools/iana-sync`: removed `toml` and `serde` dependencies
- `tools/iana-sync`: `TAG_NAME_OVERRIDES` static array removed; `load_tag_name_overrides()` now reads `iana-cbor/tag_names.toml` — fixes DRY violation (naming data belongs to the crate, not the tool)
- `tools/iana-sync`: added `toml = "0.8"` and `serde` dependencies for TOML parsing
- `tools/iana-sync`: `constant_name_for_tags`, `constant_name`, `append_constants`, `update_registry`, `check_registry` now accept an explicit `overrides: &BTreeMap<i128, String>` parameter
- `iana-cbor`: `simple_values` constants changed from `u64` to `u8` — CBOR simple values are capped at 255 by spec and embedded decoders return `u8`; avoids redundant cast at call site
- `iana-cbor`: `timescales` constants changed from `u64` to `u8` — same rationale; only values 0 and 1 are assigned
- `tools/iana-sync`: added `IntegerType::U8` variant; `parse_label` and registry entries updated accordingly
- `tools/iana-sync`: removed `pub mod label` wrapper from all registry modules — constants are now at module level (e.g. `iana_suit::commands::FETCH` instead of `iana_suit::commands::label::FETCH`)
- `iana-suit`: label type changed from `i64` to `i32` — aligns with the signed integer range of SUIT CBOR map keys and avoids 64-bit software emulation on 32-bit MCU targets
- `tools/iana-sync`: redesigned `check` command to use compiled `is_known()` predicates instead of source-text parsing — eliminates false positives from formatting changes
- `tools/iana-sync`: added HTTP HEAD fast-check using `Last-Modified` header — avoids full CSV download when registry snapshot is current
- `tools/iana-sync`: added 15-second timeout on all HTTP requests
- `tools/iana-sync`: added path dependencies on `iana-suit` and `iana-cbor` for compiled-constant validation

### Removed
- `metadata.rs` modules from `iana-suit`, `iana-cbor`, and `iana-rats` — replaced by a single `IANA_SNAPSHOT: &str` constant at crate root
- Redundant re-exports `PACKAGE_NAME`, `PACKAGE_VERSION`, `VERSION`, `IANA_REGISTRY_SNAPSHOT` — use `env!("CARGO_PKG_NAME")` / `env!("CARGO_PKG_VERSION")` directly
- `[package.metadata.iana]` tables from all `Cargo.toml` files — `IANA_SNAPSHOT` in `src/lib.rs` is the single source of truth

---

## [0.1.0] — Unreleased

### Added

#### `iana-suit`
- 11 SUIT registry modules fully populated from IANA snapshot 2026-02-17: `envelope`, `manifest`, `common`, `commands`, `parameters`, `text`, `component_text`, `report`, `record`, `report_reasons`, `capability_report`
- All labels typed as `i32`
- `is_known(label: i32) -> bool` `const fn` predicate in every module
- `IANA_SNAPSHOT: &str` constant at crate root
- Exhaustive per-module regression tests for all constant values
- `#![no_std]`, `#![forbid(unsafe_code)]`

#### `iana-cbor`
- 4 CBOR registry modules fully populated from IANA snapshot 2026-07-20: `simple_values`, `tags`, `timescales`, `time_tag_map_keys`
- `tags` typed as `u64` (IANA has assigned tags above `u32::MAX`); `simple_values` and `timescales` typed as `u8` (spec-capped at 255, matches embedded decoder return type); `time_tag_map_keys` typed as `i128` (values outside `i64` range)
- Hand-named constants: `tags::SUIT_ENVELOPE = 107`, `tags::SUIT_MANIFEST = 1070`, `simple_values::FALSE = 20`, `simple_values::TRUE = 21`, `simple_values::NULL = 22`, `simple_values::UNDEFINED = 23`
- `is_known()` predicate in every module
- `IANA_SNAPSHOT: &str` constant at crate root
- `#![no_std]`, `#![forbid(unsafe_code)]`

#### `iana-rats`
- Skeleton crate with `eat_intended_uses` and `cmw_indicators` module stubs
- Constants pending IANA CSV availability (tracked in [ROADMAP.md](ROADMAP.md) — v0.3.0)
- `IANA_SNAPSHOT: &str` constant at crate root
- `#![no_std]`, `#![forbid(unsafe_code)]`

#### `tools/iana-sync`
- `check` command: downloads IANA CSVs and validates all 15 registries against compiled `is_known()` predicates
- `update` command: appends missing constants to registry source files; never removes stale entries
- Stable-name override map for hand-named constants (e.g. CBOR tag 107 → `SUIT_ENVELOPE`)
- Per-registry `snapshot` field for HEAD fast-check
- Unit tests for `parse_csv`, `source_labels`, `constant_name`, `parse_last_modified_date`

#### Workspace
- GitHub Actions workflow: PR validation (`check`, `clippy`, `cargo doc`, `thumbv7em-none-eabihf` no_std check) and weekly scheduled sync with automatic PR creation
- `LICENSE-MIT` and `LICENSE-APACHE` at workspace root
- `AGENTS.md` — development conventions for AI-assisted and human contributors
- `rust-version = "1.85"`, `repository`, `keywords`, `categories` in all published crate manifests
- `[workspace.lints]`: `missing_docs = "warn"`, `clippy::pedantic = "warn"`
- Per-crate `README.md` files
