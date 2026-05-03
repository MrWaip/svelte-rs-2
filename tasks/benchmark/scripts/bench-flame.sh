#!/usr/bin/env bash
set -euo pipefail

PATH_ARG="${1:?usage: bench-flame.sh <path> [extra flags forwarded to profile bin]}"
shift
ROOT=tasks/benchmark/benches/compiler
OUT_BASE=profile/one
SCRIPTS=tasks/benchmark/scripts
DURATION=10

mode_label=prod
for a in "$@"; do
    [ "$a" = "--dev" ] && mode_label=dev
done

rel="${PATH_ARG#$ROOT/}"
slug=$(echo "$rel" | tr '/' '_' | sed -E 's/\.svelte(\.js)?$//')
DIR="$OUT_BASE/${slug}_${mode_label}"

rm -rf "$OUT_BASE"
mkdir -p "$DIR"

samply record \
    --save-only --no-open --unstable-presymbolicate \
    -d "$DURATION" \
    -o "$DIR/profile.json.gz" \
    -- target/profiling/profile "$PATH_ARG" "$DURATION" "$@"

gunzip -f "$DIR/profile.json.gz"
node "$SCRIPTS/gecko-to-folded.mjs" "$DIR"
echo "done. read: $DIR/top.txt"
