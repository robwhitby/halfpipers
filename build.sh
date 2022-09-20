#!/usr/bin/env bash
set -euo pipefail

fmt_opts=""
[ "${CI-}" = "true" ] && fmt_opts="-- --check"

export CARGO_TERM_COLOR=always

cargo fmt --all $fmt_opts
cargo build
cargo test

echo Finished!