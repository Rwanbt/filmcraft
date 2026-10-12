# Remaining risks — production readiness ledger

Reference: `main@4e2d0bc2d3646f9cf27ef7aa8b2095c6858c7efe`; first CI-improved commit `4aab399a0a1175ad7691e01ccec2ffdc716a955a`; reviewed 2026-10-12.

| Priority | Risk / evidence | Current state | Required action |
| --- | --- | --- | --- |
| P0 | Native startup and crashes, especially OS/GPU-specific conditions (existing `docs/gaps.md` G4, G9) | **UNVERIFIED** | Run real binaries and smoke tests on Linux, Windows and macOS; investigate reported crashes and safe-mode fallback |
| P0 | Corruption/loss of user work via media import, edits, export, autosave and recovery | **UNVERIFIED** in new CI | Execute engine/project/interchange full suites; realistic import→edit→save→reopen→export corpus; verify checksums and metadata |
| P0 | Untrusted bitstreams and parser limits | **PARTIAL** (bitstream has only 76.17% instrumented lines) | Missing branches, fuzz/mutation/truncation tests with fixed seeds and reproducer corpus; extend to all codecs/containers |
| P1 | Lack of repeatable full workspace PR gates | **OPEN**; only two foundation crates currently gated | Stage `cargo test --workspace --locked`, Clippy, WASM, platform matrix and collect actual failure logs; avoid secret-dependent steps |
| P1 | Partial media format/codec coverage, including 10-bit / 4:2:2 formats (`docs/gaps.md` G3, G10) | **UNVERIFIED** | Pinned open fixture corpus, ffmpeg/ffprobe external oracles, CPU/hardware parity and negative cases |
| P1 | Actual desktop and browser GUI user journeys | **UNVERIFIED** | Native launch, project/import/edit/undo/save/reopen/export, browser navigation, errors and responsive tests |
| P1 | Coverage extrapolation from two small crates | **NOT ALLOWED** | Collect measured LCOV separately per crate; branch metrics require supported tooling; do not mix denominators |
| P2 | Determinism under races, clocks, network and hardware | **UNVERIFIED** | Controlled clocks/seeds, repeat CI suites, locale/timezone tests, bounded concurrency, CPU/GPU tolerances |
| P2 | Main branch is not protected (GitHub branch metadata on audit) | **OPEN** | Require successful checks and reviews using repository settings before merging after authorization |

All risks outside the two directly tested crates remain present until their own checks run; the existence of test files does not close them. This audit adds no upstream PR, no releases, no secret access and no automatic main integration.
