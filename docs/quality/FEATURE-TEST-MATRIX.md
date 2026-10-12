# Feature / test traceability — initial code-based inventory

Reference tree: `main@4e2d0bc2d3646f9cf27ef7aa8b2095c6858c7efe`, inspected 2026-10-12.

## Functional traceability (priority slice)

| ID | Contract / risk | Component | Behavioral evidence | Runner / state |
| --- | --- | --- | --- | --- |
| FC-TIME-001 | Empty, zero-duration or reversed half-open ranges never overlap; nonempty overlaps symmetric and consistent with intersection | `filmcraft-time` | `crates/time/tests/range_contract.rs` — three tests, exhaustive bounded oracle | Linux run 38268006579: **VERIFIED for this contract** |
| FC-TIME-002 | Frame↔tick and timecode conversions respect exact rational clocks and invalid-rate boundaries | `filmcraft-time` | Existing Rust and proptest suites; scope incomplete and branch coverage absent | Linux foundation success; **PARTIAL** |
| FC-BITS-001 | Bitstream, Exp-Golomb and RBSP operations handle valid and hostile inputs with correct roundtrip | `filmcraft-bitstream` | Existing tests; coverage 227/298 lines | Linux foundation success; **PARTIAL** |
| FC-EDIT-001 | Timeline insert/trim/razor, undo/redo and sequence invariants do not lose data | `edit`, `engine` | Existing `crates/engine/src/*tests.rs`, `crates/edit/src/tests.rs` | Full workspace not run: **UNVERIFIED** |
| FC-PERSIST-001 | Save, autosave, crash recovery, roundtrip and journaling preserve projects | `engine`, `project` | `crates/engine/src/autosave_tests.rs` | Not executed by new quality gate: **UNVERIFIED** |
| FC-DECODE-001 | Damaged media/container data never crashes parsers and decoders | `codecs` and L0 codec/container crates | Oracle, conformance and robustness tests present | Not executed: **UNVERIFIED** |
| FC-EXPORT-001 | Render→encode→mux respects video/audio timing, integrity and output metadata | `render`, `export`, `codecs` | Existing engine/export and codec test files | No complete CI execution: **UNVERIFIED** |
| FC-UI-001 | Real controls: menus, timeline, media placement, keyboard, undo and panels | `ui-egui`, `engine` | Headless `egui_kittest` scripted UI tests exist | Headless suite not run; native GUI E2E absent from new gate: **UNVERIFIED** |
| FC-CLI-001 | CLI/MCP commands accept valid params, reject invalid params, and preserve state | `filmcraft-cli`, `automation` | CLI and engine tests in repository | New gate not run: **UNVERIFIED** |
| FC-WEB-001 | WASM startup, OPFS restore, media interaction and WebCodecs fallbacks | `filmcraft-web` | Some web tests and build support exist | Browser/E2E not executed: **UNVERIFIED** |
| FC-PLAT-001 | Safe fallback and startup on Linux, Windows, macOS | `platform`, desktop app | Platform/hardware test files exist | Device OS/hardware matrix not executed: **UNVERIFIED** |

A passing low-level unit test does **not** certify a corresponding end-to-end user workflow. The table records scoped proof only.

## Workspace manifest census

A heuristic count of files with `/tests/`, `tests.rs`, or `*_tests.rs` names is shown below. It includes helpers and misses inline `#[cfg(test)]` test functions. It therefore measures **neither test case count nor coverage**. All unexecuted modules remain `UNVERIFIED` regardless of test file presence.

| Package | Source path | Candidate test-related file paths | Scope state |
| --- | --- | ---: | --- |
| `filmcraft-cli` | `apps/filmcraft-cli` | 1 | UNVERIFIED |
| `filmcraft-web` | `apps/filmcraft-web` | 5 | UNVERIFIED |
| `filmcraft` | `apps/filmcraft` | 0 | UNVERIFIED |
| `filmcraft-aac` | `crates/aac` | 4 | UNVERIFIED |
| `filmcraft-ac3` | `crates/ac3` | 2 | UNVERIFIED |
| `filmcraft-apv` | `crates/apv` | 2 | UNVERIFIED |
| `filmcraft-audio-dsp` | `crates/audio-dsp` | 2 | UNVERIFIED |
| `filmcraft-automation` | `crates/automation` | 0 | UNVERIFIED |
| `filmcraft-av1` | `crates/av1` | 6 | UNVERIFIED |
| `filmcraft-bitstream` | `crates/bitstream` | 0 | PARTIAL |
| `filmcraft-captions` | `crates/captions` | 1 | UNVERIFIED |
| `filmcraft-cfb` | `crates/cfb` | 1 | UNVERIFIED |
| `filmcraft-codecs` | `crates/codecs` | 16 | UNVERIFIED |
| `filmcraft-color` | `crates/color` | 0 | UNVERIFIED |
| `filmcraft-dnx` | `crates/dnx` | 5 | UNVERIFIED |
| `filmcraft-edit` | `crates/edit` | 3 | UNVERIFIED |
| `filmcraft-engine` | `crates/engine` | 58 | UNVERIFIED |
| `filmcraft-export` | `crates/export` | 7 | UNVERIFIED |
| `filmcraft-format` | `crates/format` | 7 | UNVERIFIED |
| `filmcraft-frame` | `crates/frame` | 1 | UNVERIFIED |
| `filmcraft-geom` | `crates/geom` | 0 | UNVERIFIED |
| `filmcraft-golden` | `crates/golden` | 1 | UNVERIFIED |
| `filmcraft-gpu` | `crates/gpu` | 4 | UNVERIFIED |
| `filmcraft-h264` | `crates/h264` | 6 | UNVERIFIED |
| `filmcraft-h264enc` | `crates/h264enc` | 3 | UNVERIFIED |
| `filmcraft-hevc` | `crates/hevc` | 5 | UNVERIFIED |
| `filmcraft-interchange` | `crates/interchange` | 12 | UNVERIFIED |
| `filmcraft-isobmff` | `crates/isobmff` | 6 | UNVERIFIED |
| `filmcraft-matroska` | `crates/matroska` | 3 | UNVERIFIED |
| `filmcraft-media` | `crates/media` | 0 | UNVERIFIED |
| `filmcraft-mpeg2v` | `crates/mpeg2v` | 5 | UNVERIFIED |
| `filmcraft-mpegts` | `crates/mpegts` | 1 | UNVERIFIED |
| `filmcraft-mxf` | `crates/mxf` | 2 | UNVERIFIED |
| `filmcraft-ogg` | `crates/ogg` | 1 | UNVERIFIED |
| `filmcraft-opus` | `crates/opus` | 4 | UNVERIFIED |
| `filmcraft-platform` | `crates/platform` | 24 | UNVERIFIED |
| `filmcraft-project` | `crates/project` | 0 | UNVERIFIED |
| `filmcraft-prores` | `crates/prores` | 6 | UNVERIFIED |
| `filmcraft-render` | `crates/render` | 10 | UNVERIFIED |
| `filmcraft-scopes` | `crates/scopes` | 1 | UNVERIFIED |
| `filmcraft-speech` | `crates/speech` | 1 | UNVERIFIED |
| `filmcraft-testkit` | `crates/testkit` | 0 | UNVERIFIED |
| `filmcraft-text` | `crates/text` | 0 | UNVERIFIED |
| `filmcraft-time` | `crates/time` | 1 | PARTIAL |
| `filmcraft-tts-text` | `crates/tts-text` | 0 | UNVERIFIED |
| `filmcraft-tts` | `crates/tts` | 1 | UNVERIFIED |
| `filmcraft-ui-egui` | `crates/ui-egui` | 64 | UNVERIFIED |
| `filmcraft-vp9` | `crates/vp9` | 5 | UNVERIFIED |
| `xtask` | `xtask` | 0 | UNVERIFIED |

## Completion criteria for each package

Determine public contracts and error boundaries; run collected unit, integration and E2E targets as applicable; collect per-file lines/functions/branches; add hostile-input regressions and media oracles; repeat deterministic suites; link exact SHA/job; record justified exclusions and residual risks. Transition to `VERIFIED` only when evidence exists.
