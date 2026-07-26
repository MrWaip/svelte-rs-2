#!/usr/bin/env bash
set -euo pipefail

# Profile-guided optimization pipeline.
#   scripts/pgo.sh napi     — optimize the published napi addon
#   scripts/pgo.sh bench    — optimize target/release/bench_once
#
# The instrumented build and the optimized build must come from the same cargo
# invocation: symbol names carry a per-unit disambiguator, and a profile taken
# from a different invocation matches nothing.
#
# Extra training corpora: PGO_CORPUS="/path/to/app /path/to/other"
# Training rounds:        PGO_ROUNDS=3

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
DATA_DIR="$ROOT/target/pgo-data"
PROFDATA="$DATA_DIR/merged.profdata"
ROUNDS="${PGO_ROUNDS:-3}"
LLVM_PROFDATA="$(rustc --print target-libdir)/../bin/llvm-profdata"

target="${1:-napi}"

if [ ! -x "$LLVM_PROFDATA" ]; then
    echo "llvm-profdata not found at $LLVM_PROFDATA" >&2
    echo "install it with: rustup component add llvm-tools-preview" >&2
    exit 1
fi

build_napi() {
    npm run --prefix "$ROOT/packages/svelte-rs2" build:release
}

build_bench() {
    cargo build --release -p benchmark --bin bench_once
}

rm -rf "$DATA_DIR"
mkdir -p "$DATA_DIR"

case "$target" in
napi)
    echo "==> instrumented napi build"
    RUSTFLAGS="-Cprofile-generate=$DATA_DIR" build_napi
    echo "==> training"
    PGO_ROUNDS="$ROUNDS" node "$ROOT/scripts/pgo-train.mjs"
    ;;
bench)
    echo "==> instrumented trainer build"
    RUSTFLAGS="-Cprofile-generate=$DATA_DIR" cargo build --release -p benchmark --bin pgo_train
    echo "==> training"
    # shellcheck disable=SC2086
    "$ROOT/target/release/pgo_train" "$ROUNDS" ${PGO_CORPUS:-}
    ;;
*)
    echo "usage: scripts/pgo.sh [napi|bench]" >&2
    exit 1
    ;;
esac

echo "==> merging counters"
"$LLVM_PROFDATA" merge -o "$PROFDATA" "$DATA_DIR"

echo "==> optimized build"
case "$target" in
napi) RUSTFLAGS="-Cprofile-use=$PROFDATA" build_napi ;;
bench) RUSTFLAGS="-Cprofile-use=$PROFDATA" build_bench ;;
esac
