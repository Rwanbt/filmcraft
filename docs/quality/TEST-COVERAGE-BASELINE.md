# Test coverage baseline — FilmCraft fork

- Measured: **2026-10-12** (GitHub Actions, Ubuntu 24.04, Rust stable, `cargo-llvm-cov`).
- Baseline commit: `4aab399a0a1175ad7691e01ccec2ffdc716a955a` (branch `test/filmcraft-deterministic-baseline-20261012`).
- Base `main`: `4e2d0bc2d3646f9cf27ef7aa8b2095c6858c7efe`.
- Evidence: [Actions run 38268006579](https://github.com/Rwanbt/filmcraft/actions/runs/38268006579), artifact `foundation-coverage` (ID `11755512467`, `coverage.lcov`, SHA-256 `cf6ac12344cb1c351d8547b6ef225ccc4e9550b32f61b2e64c2d429e3e35cba8`).
- Baseline command: `cargo llvm-cov --locked -p filmcraft-time -p filmcraft-bitstream --lcov --output-path coverage.lcov`.
- Instrumentation scope: **two crates only**; this is **not whole-workspace coverage**.

| Crate | Covered lines / total | Lines | Covered functions / total | Functions |
| --- | ---: | ---: | ---: | ---: |
| `filmcraft-time` | 491 / 517 | 94.97% | 65 / 72 | 90.28% |
| `filmcraft-bitstream` | 227 / 298 | 76.17% | 28 / 43 | 65.12% |
| **Instrumented subtotal** | **718 / 815** | **88.10%** | **93 / 115** | **80.87%** |

Branch coverage: **not reported** by this LCOV artifact (`BRF/BRH` absent/zero); do not infer 100% or 0%. There is no justifiable coverage threshold yet. Extend to every applicable crate after stable full workspace execution, with separately documented exclusions.

## Inventory (not measured coverage)

The main commit's recursive Git tree contains 1,467 entries (1,236 file blobs), 884 `.rs` blobs, and 49 workspace member manifests: **45 crates + 3 applications + xtask**. About 286 candidate test-related Rust paths were found by filename/location heuristics; one new integration test has been added on this branch. This count includes helpers and is **not a count of collected tests**. An accurate workspace test count requires `cargo test --workspace -- --list` in CI.

## Proven regression

- `TimeRange::overlaps` returned true for an empty or reversed range overlapping a nonempty one.
- Pre-fix verification: [Actions run 38267840685](https://github.com/Rwanbt/filmcraft/actions/runs/38267840685); the `range_contract` runner reported **2 failed, 1 passed** after formatting was fixed.
- Fix: reject ranges where `duration <= 0` before testing half-open overlap.
- Post-fix: [Actions run 38268006579](https://github.com/Rwanbt/filmcraft/actions/runs/38268006579); formatting, 3 new regression tests, foundation tests, Clippy and coverage job succeeded.
- The bounded oracle exercises 1,764 combinations of two short ranges; it checks symmetry and intersection consistency.

## Current limits

No verified cross-platform run (Windows/macOS), native GUI startup, browser E2E, full workspace test census, fuzz coverage, ABI-specific hardware media path, codec corpus or long-term determinism/repetition baseline. These are explicitly **unverified**, not passed.

Next: expand CI by risk, gather per-crate instrumented numbers, add hostile bitstream tests and run full workspace gates where feasible. See [FEATURE-TEST-MATRIX.md](FEATURE-TEST-MATRIX.md) and [REMAINING-RISKS.md](REMAINING-RISKS.md).
