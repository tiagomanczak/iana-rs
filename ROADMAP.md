# iana-rs Roadmap

This roadmap tracks milestones for the `iana-rs` workspace. Each milestone has a
dedicated GitHub issue for progress tracking.

Leaf crates (`iana-suit`, `iana-cbor`, etc.) are versioned independently. Milestone
numbers below reflect feature groupings, not lockstep crate version bumps.

---

## P0 — Pre-publication hardening

> Prerequisites before any crates.io release. Nothing ships until all items here are done.

- [ ] Set `iana-rats` to `publish = false` until constants are populated
- [ ] Add `rustfmt.toml` to workspace root
- [ ] Replace all draft RFC identifiers with final RFC numbers where published; flag remaining drafts explicitly
- [ ] Document cross-registry label overlap (same integer reused across SUIT registries by design)
- [ ] Add stable-name override map to `iana-sync` — prevents IANA description changes from silently renaming Rust constants
- [ ] Fix `iana-sync` fast-check: `Last-Modified` skip must fall back to full CSV hash comparison, not trust date alone
- [ ] Add `cargo package --dry-run` and `cargo publish --dry-run` to CI for all published crates
- [ ] Document MSRV policy: Edition 2024, Rust 1.85 minimum; CI runs on MSRV and stable; MSRV bumps are minor releases

**Acceptance criteria:** CI green on MSRV and stable. `cargo publish --dry-run` passes
for `iana-suit` and `iana-cbor`.

---

## v0.1.0 — First public release

> Publish `iana-suit` and `iana-cbor` to crates.io. `iana-rats` stays `publish = false`.

- [ ] `iana-suit` 0.1.0 — 11 SUIT registries, fully populated, `i32` labels
- [ ] `iana-cbor` 0.1.0 — 4 CBOR registries (`u64` tags/simple values, `i128` time map keys)
- [ ] `iana-sync` — all 15 registries verified, stable-name override map in place
- [ ] CI: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo doc -D warnings`,
  `cargo test --workspace`, `cargo check --target thumbv7em-none-eabihf -p iana-suit -p iana-cbor`

---

## v0.2.0 — COSE

> Tightest coupling to SUIT — required by any SUIT manifest signer or verifier.

New crate: `iana-cose` (`publish = true`).

- [ ] COSE Header Parameters — RFC 9052 §3.1 (`alg`, `crit`, `kid`, `IV`, counter signature, etc.)
- [ ] COSE Algorithms — RFC 9053 (signing, MAC, encryption algorithm identifiers)
- [ ] COSE Key Common Parameters — RFC 9053 §7
- [ ] COSE Key Type Parameters — RFC 9053 §7
- [ ] COSE Key Types
- [ ] COSE Header Algorithm Parameters
- [ ] COSE Elliptic Curves
- [ ] Add all `iana-cose` registries to `iana-sync` REGISTRIES
- [ ] Add `iana-cose` to CI target checks

---

## v0.3.0 — CWT / EAT + RATS constants

> Completes the RATS remote attestation story. EAT reuses CWT Claims — document this
> relationship explicitly; there is no independent EAT Claims registry.

New crate: `iana-cwt` (`publish = true`). Unblock `iana-rats` (`publish = true`).

- [ ] `iana-cwt` — CWT Claims (RFC 8392 §9.1)
- [ ] `iana-cwt` — CWT Confirmation Methods
- [ ] `iana-cwt` — CWT Status Mechanisms
- [ ] Populate `iana-rats::eat_intended_uses` and `cmw_indicators` from IANA CSV
  (closes BLOCKER: empty skeleton modules)
- [ ] Add `is_known()` predicates to all newly populated `iana-rats` modules
- [ ] Add `iana-cwt` and `iana-rats` to `iana-sync` REGISTRIES
- [ ] First release of `iana-rats` to crates.io

---

## v0.4.0 — CoAP

> SUIT's primary transport protocol.

New crate: `iana-coap` (`publish = true`).

- [ ] Option Numbers — RFC 7252 §12.2
- [ ] Content-Formats — RFC 7252 §12.3 + RFC 9345 SUIT additions
- [ ] Response Codes
- [ ] Method Codes
- [ ] Signaling Codes
- [ ] Signaling Option Numbers
- [ ] Add `iana-coap` to `iana-sync` REGISTRIES

---

## v0.5.0 — TLS / DTLS

> Secure transport for constrained devices.

New crate: `iana-tls` (`publish = true`).

**Scope policy:** expose all assigned IANA values in all listed registries, including
deprecated entries. Deprecation status is documented in each constant's doc comment.
`is_known()` means "assigned in IANA", not "safe to negotiate". Recommended-algorithm
profiles for embedded targets are a consumer concern, not a registry filtering concern.

- [ ] Cipher Suites (DTLS 1.2/1.3 profile first; full registry via `iana-sync`)
- [ ] ExtensionType
- [ ] ContentType
- [ ] Alerts
- [ ] HandshakeType
- [ ] Supported Groups
- [ ] SignatureScheme
- [ ] EC Point Formats / EC Curve Types
- [ ] PSK Key Exchange Modes
- [ ] Add all `iana-tls` registries to `iana-sync`

---

## v0.6.0 — `iana-rs` facade crate

> Deferred until all leaf crates have stable published versions. The facade has optional
> dependencies only — leaf crates retain the zero-dependency invariant.

New crate: `iana-rs` (`publish = true`).

- [ ] Feature-gated re-exports: `features = ["suit", "cbor", "cose", "rats", "cwt", "coap", "tls"]`
- [ ] `default = []` — no features enabled by default
- [ ] No wildcard re-exports; every public path is explicit
- [ ] All feature combinations tested in CI
- [ ] Document: enabling a feature adds a transitive dependency

---

## v1.0.0 — Stable API

> Not the first release — a compatibility commitment. All of the following must be true.

### Content
- [ ] No skeleton or `publish = false` crates in the supported scope
- [ ] All declared registries fully populated and sync-verified at a pinned snapshot

### API contract
- [ ] Public module paths, constant names, numeric types, and `is_known()` signatures are frozen
- [ ] Stable-name override map covers all generated names — IANA description changes cannot silently rename a constant
- [ ] Deprecated IANA assignments kept with `#[deprecated]` doc annotations; never silently removed

### Tooling and CI
- [ ] `iana-sync` uses content hash comparison, not `Last-Modified` alone
- [ ] Pinned IANA CSV snapshots available for offline/reproducible CI runs
- [ ] `cargo semver-checks` (or equivalent) run on every PR

### Release process
- [ ] `cargo package --list` and `cargo publish --dry-run` in CI for all crates
- [ ] Documented deprecation and MSRV bump policy
- [ ] At least one known downstream consumer validated against the release

---

## Registry dependency graph

```
CBOR
 └── COSE  ──────────────────────────────────┐
      └── SUIT (authentication wrapper)      │
      └── CWT/EAT (attestation token)        │
           └── RATS                          │
CoAP (transport)  ◄──────────────────────────┘
TLS/DTLS (secure transport)
```

---

## Versioning policy

| Change | Version bump |
|--------|-------------|
| Documentation fixes only | Patch |
| New constants added | Minor |
| Constants renamed, removed, or retyped | Major |
| MSRV increase | Minor |

Each leaf crate is versioned independently. A new COSE assignment does not bump
`iana-suit`.
