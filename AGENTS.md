# Project Instructions — JBC Edition 1.3 Rust Workspace

## Methodology & Architecture
- Govern engineering work via **Permit-and-Contract Engineering (`PCE`)** and **Jezz Software Building Code (`JBC` Edition 1.3)** configured in [`jezz.toml`](jezz.toml). Never frame deliverables around Agile user stories (`As a ..., I want ...`), "SDD", or Markdown task checklists (`- [ ]`).
- Maintain strict compartment isolation enforced by [`blueprint.json`](blueprint.json) Layout-vs-Schematic (`LVS`):
  - **`src/domain/`** — Closed Serde `#[serde(deny_unknown_fields)]` structs, explicit `Result<T, E>` propagation (`0` `.unwrap()` / `.expect()` calls in production code), and Z3-provable `/// @requires` / `/// @ensures` doc contracts paired with runtime guards.
  - **`src/gateway/`** — Bounded wire decoders preserving causal error context and routing strictly through `crate::domain`'s public exports.
- Never hardcode host filesystem paths (`JBC-SEC-006`) or introduce `unsafe` without permit waiver.

## Statutory Verification Commands
- **Cargo Build, Clippy, & Property Suite:**
  `cargo test`
- **JBC Edition 1.3 Static Inspection:**
  `uvx --from git+https://github.com/futileandstupid/jezz@v0.6.0 jezz inspect .`
- **Z3 Deductive Contract Proof Discharge:**
  `uvx --from git+https://github.com/futileandstupid/jezz@v0.6.0 jezz prove . --require-deductive`
- **Blueprint CAD & LVS Equivalence Gate (Refresh `blueprint.json` When Modifying Boundaries):**
  `uvx --from git+https://github.com/futileandstupid/jezz@v0.6.0 jezz blueprint . --verify-target blueprint.json`
