# Test execution & GitHub Actions evidence

Fork: `Rwanbt/filmcraft`, branch `test/filmcraft-deterministic-baseline-20261012`, baseline parent `4aab399a0a1175ad7691e01ccec2ffdc716a955a`.

## Commands executed by GitHub Actions

```sh
cargo fmt --all -- --check
cargo test --locked -p filmcraft-time --test range_contract
cargo test --locked -p filmcraft-time -p filmcraft-bitstream
cargo clippy --locked -p filmcraft-time -p filmcraft-bitstream --all-targets -- -D warnings
cargo llvm-cov --locked -p filmcraft-time -p filmcraft-bitstream --lcov --output-path coverage.lcov
```

The selected Linux foundation job on [run 38268006579](https://github.com/Rwanbt/filmcraft/actions/runs/38268006579) **passed**; coverage artifact attached. Prior to code correction, the actual regression runner on [run 38267840685](https://github.com/Rwanbt/filmcraft/actions/runs/38267840685) **failed exactly two contract tests**. [Packaging lint run 38267846141](https://github.com/Rwanbt/filmcraft/actions/runs/38267846141) passed on the pre-fix formatting commit; later commits require their own SHA-specific checks.

The local analysis container had neither `rustc` / `cargo` nor DNS access to GitHub. Consequently, **no local Rust tests are claimed**. The GitHub-hosted runners provide the execution evidence.

## Optional deeper validation defined but not yet run

```sh
FILMCRAFT_REQUIRE_ORACLES=1 cargo test --workspace --locked
cargo clippy --workspace --locked --all-targets -- -D warnings
cargo xtask layers
cargo xtask assets
cargo xtask wasm
```

The full workspace gate is currently configured for a manual `workflow_dispatch` event to avoid automatically spending large cross-platform build resources before the foundation baseline is stable. Its omission is not success. The source project's `cargo xtask ci` also runs comprehensive gates.

## Reproduction and safe recovery

To reproduce the interval regression independently, check out `5f6bdb37f371a793ed3cf170615d63bb7aed6479` and run `cargo test --locked -p filmcraft-time --test range_contract`: two assertions failed in the recorded CI. The fix is first present in `4aab399a0a1175ad7691e01ccec2ffdc716a955a`. Do not checkout another SHA and attribute its result to that commit.

Do not enable 100% enforcement or a full-workspace production-ready badge until instrumented results justify them; retain draft PR, do not merge without authorization.
