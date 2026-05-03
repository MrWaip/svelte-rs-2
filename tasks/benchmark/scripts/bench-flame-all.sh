#!/usr/bin/env bash
set -euo pipefail

ROOT=tasks/benchmark/benches/compiler
OUT_BASE=profile/all
SCRIPTS=tasks/benchmark/scripts
DURATION=10

rm -rf "$OUT_BASE"
mkdir -p "$OUT_BASE"

run_case() {
    local f="$1"
    local mode_label="$2"
    shift 2
    local rel="${f#$ROOT/}"
    local slug=$(echo "$rel" | tr '/' '_' | sed -E 's/\.svelte(\.js)?$//')
    local dir="$OUT_BASE/${slug}_${mode_label}"
    mkdir -p "$dir"
    echo ">>> $mode_label  $rel"
    samply record \
        --save-only --no-open --unstable-presymbolicate \
        -d "$DURATION" \
        -o "$dir/profile.json.gz" \
        -- target/profiling/profile "$f" "$DURATION" "$@" >/dev/null
    gunzip -f "$dir/profile.json.gz"
    node "$SCRIPTS/gecko-to-folded.mjs" "$dir" >/dev/null
}

while IFS= read -r f; do
    run_case "$f" prod
    run_case "$f" dev --dev
done < <(find "$ROOT" -type f \( -name '*.svelte' -o -name '*.svelte.js' \) | sort)

node "$SCRIPTS/aggregate-folded.mjs" "$OUT_BASE"
echo "done. read: $OUT_BASE/aggregate.top.txt"
