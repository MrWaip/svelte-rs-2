#!/usr/bin/env bash
set -euo pipefail

# Build the published napi addon for one target.
#   NAPI_TARGET  — rust target triple (required)
#   NAPI_CROSS   — napi-cross | zig | empty
#   PGO          — non-empty to train a profile first; only valid when the
#                  produced addon runs on this runner
#
# PGO never blocks a release: if any step of it fails, the plain build runs.

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
DATA_DIR="$ROOT/target/pgo-data"
PROFDATA="$DATA_DIR/merged.profdata"
ROUNDS="${PGO_ROUNDS:-1}"

cd "$ROOT"

cross_flag=""
case "${NAPI_CROSS:-}" in
napi-cross) cross_flag="--use-napi-cross" ;;
zig) cross_flag="--cross-compile" ;;
esac

build() {
    # shellcheck disable=SC2086
    node_modules/.bin/napi build \
        --manifest-path crates/napi_compiler/Cargo.toml \
        -p napi_compiler \
        --package-json-path packages/svelte-rs/package.json \
        --platform \
        --output-dir packages/svelte-rs/compiler/native \
        --js binding.cjs \
        --dts binding.d.ts \
        --release \
        --target "$NAPI_TARGET" \
        $cross_flag \
        "$@"
}

find_profdata() {
    local libdir
    libdir=$(rustc --print target-libdir)
    if command -v cygpath >/dev/null 2>&1; then
        libdir=$(cygpath -u "$libdir")
    fi
    for candidate in "$libdir/../bin/llvm-profdata" "$libdir/../bin/llvm-profdata.exe"; do
        if [ -x "$candidate" ]; then
            echo "$candidate"
            return 0
        fi
    done
    return 1
}

train() {
    local profdata
    profdata=$(find_profdata) || {
        echo "llvm-profdata not found — needs rustup component add llvm-tools-preview" >&2
        return 1
    }

    rm -rf "$DATA_DIR"
    mkdir -p "$DATA_DIR"

    echo "==> instrumented build"
    RUSTFLAGS="-Cprofile-generate=$DATA_DIR" build

    echo "==> training"
    PGO_ROUNDS="$ROUNDS" node "$ROOT/scripts/pgo-train.mjs"

    echo "==> merging counters"
    "$profdata" merge -o "$PROFDATA" "$DATA_DIR"
}

if [ -n "${PGO:-}" ] && train; then
    echo "==> optimized build"
    RUSTFLAGS="-Cprofile-use=$PROFDATA" build --strip
else
    if [ -n "${PGO:-}" ]; then
        echo "==> profile unavailable, building without it" >&2
    fi
    build --strip
fi
