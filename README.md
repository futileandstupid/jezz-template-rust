# `jezz-template-rust` — JBC Edition 1.3 Starter

Turnkey **Rust (`edition = "2021"`)** starter repository governed by the **Jezz Software Building Code (`JBC` Edition 1.3)** and **Permit-and-Contract Engineering (`PCE`)**, configured via [`jezz.toml`](jezz.toml).

## Architectural Layout (`domain` + `gateway`)

- **`src/domain/mod.rs`** — Closed Serde `#[serde(deny_unknown_fields)]` wire structs, zero-unwrap `Result<T, SettlementError>` propagation, and Z3-discharged `net_after_fee` (`/// @ensures` + `debug_assert!`).
- **`src/gateway/mod.rs`** — Bounded wire JSON parser preserving causal decode diagnostics and routing through `crate::domain`.
- **`blueprint.json`** — Sealed `BlueprintGraph` netlist enforced via Layout-vs-Schematic (`LVS`).
- **`tests/settlement_prop.rs`** — Deterministic 256-case SplitMix64 conservation property sweep and unknown-field rejection suite (`cargo test`).

## Quickstart

```bash
uvx --from git+https://github.com/futileandstupid/jezz@v0.6.1 jezz init my-rust-crate --scaffold rust
cd my-rust-crate
cargo test
```

## Statutory Gates

```bash
uvx --from git+https://github.com/futileandstupid/jezz@v0.6.1 jezz inspect .
uvx --from git+https://github.com/futileandstupid/jezz@v0.6.1 jezz prove . --require-deductive
uvx --from git+https://github.com/futileandstupid/jezz@v0.6.1 jezz blueprint . --verify-target blueprint.json --write-html blueprint.html
```
