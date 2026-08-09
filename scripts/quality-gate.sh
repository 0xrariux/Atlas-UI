#!/usr/bin/env sh
set -eu

atlas_cargo_bin="${CARGO_BIN:-cargo}"

"$atlas_cargo_bin" fmt --all -- --check
"$atlas_cargo_bin" check --workspace --all-targets
"$atlas_cargo_bin" clippy --workspace --all-targets -- -D warnings
"$atlas_cargo_bin" test --workspace --all-targets
"$atlas_cargo_bin" run -p atlas-ui-tooling -- quality-gate
