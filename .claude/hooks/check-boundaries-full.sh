#!/usr/bin/env bash
# Full boundary scan across all crates.
# Usage: bash .claude/hooks/check-boundaries-full.sh
# Exit 0 = clean, Exit 1 = violations found.
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
VIOLATIONS=0

red()   { printf "\033[31m%s\033[0m\n" "$1"; }
green() { printf "\033[32m%s\033[0m\n" "$1"; }
dim()   { printf "\033[2m%s\033[0m\n" "$1"; }

check() {
  local label="$1" pattern="$2" path="$3" exclude="${4:-}"
  local args=(-rn -P "$pattern" "$path" --include='*.rs')
  if [[ -n "$exclude" ]]; then
    args+=(--exclude="$exclude")
  fi
  local matches
  matches=$(grep "${args[@]}" 2>/dev/null | grep -v 'BOUNDARY-OK' | grep -v '// TODO(oxc-visit)' || true)
  if [[ -n "$matches" ]]; then
    red "FAIL: $label"
    echo "$matches" | while read -r line; do dim "  $line"; done
    VIOLATIONS=$((VIOLATIONS + 1))
    return 1
  fi
  green "OK: $label"
  return 0
}

echo "=== Architecture Boundary Check ==="
echo ""

# ── Codegen checks ──
CODEGEN="$ROOT/crates/svelte_codegen_client/src"
TEMPLATE="$ROOT/crates/svelte_codegen_client/src/template"

echo "── Codegen ──"

# Class 1: OXC parser in codegen (excluding allowlisted files)
grep -rnP 'oxc_parser::Parser|OxcParser::new' "$CODEGEN" --include='*.rs' \
  --exclude='builder.rs' 2>/dev/null \
  | grep -v 'script/mod.rs' | grep -v 'BOUNDARY-OK' > /tmp/bc_c1 || true
if [[ -s /tmp/bc_c1 ]]; then
  red "FAIL: OXC Parser instantiation in codegen (outside builder.rs/script)"
  cat /tmp/bc_c1 | while read -r line; do dim "  $line"; done
  VIOLATIONS=$((VIOLATIONS + 1))
else
  green "OK: No OXC Parser in codegen template"
fi

# Class 2: String parsing in codegen
check "No string parsing heuristics in codegen" \
  "starts_with\s*\(\s*['\"][{\[('\"]" "$CODEGEN" || true

check "No string splitting in codegen" \
  "split\s*\(\s*['\"][,:=]['\"]" "$CODEGEN" || true

# Class 3: Expression:: in codegen template (not builder/script)
# Known existing violations are allowlisted by file — new files will be caught.
grep -rnP 'Expression::\w+' "$TEMPLATE" --include='*.rs' 2>/dev/null \
  | grep -v 'BOUNDARY-OK' \
  | grep -v 'render_tag\.rs' \
  | grep -v 'attributes\.rs' \
  | grep -v 'events\.rs' \
  | grep -v 'bind\.rs' \
  | grep -v 'snippet\.rs' > /tmp/bc_c3 || true
if [[ -s /tmp/bc_c3 ]]; then
  red "FAIL: NEW Expression:: pattern match in codegen template (not in allowlisted files)"
  cat /tmp/bc_c3 | while read -r line; do dim "  $line"; done
  VIOLATIONS=$((VIOLATIONS + 1))
else
  green "OK: No new Expression:: matching in codegen template"
fi

# Count existing violations for visibility
EXISTING_EXPR=$(grep -rnP 'Expression::\w+' "$TEMPLATE" --include='*.rs' 2>/dev/null \
  | grep -v 'BOUNDARY-OK' | wc -l || true)
if [[ "$EXISTING_EXPR" -gt 0 ]]; then
  dim "  ($EXISTING_EXPR existing Expression:: matches in allowlisted codegen files — migrate with /migrate-boundary)"
fi

# Class 4: Deep AnalysisData chaining in codegen
check "No deep AnalysisData chaining in codegen" \
  'ctx\.analysis\.\w+\([^)]*\)\.\w+\([^)]*\)\.\w+' "$CODEGEN" || true

echo ""
echo "── Analyze ──"
ANALYZE="$ROOT/crates/svelte_analyze/src"

# String-keyed lookups
check "No HashMap<String> in analyze" \
  'HashMap<\s*String\b' "$ANALYZE" || true

# FxHashSet<String> — exclude known allowlisted uses
grep -rnP 'FxHashSet<String' "$ANALYZE" --include='*.rs' 2>/dev/null \
  | grep -v 'BOUNDARY-OK' \
  | grep -v 'rest_prop_excluded' \
  | grep -v 'collect_all_symbol_names' \
  | grep -v 'with_conflicts' \
  | grep -v 'mark_rest_prop' > /tmp/bc_str || true
if [[ -s /tmp/bc_str ]]; then
  red "FAIL: FxHashSet<String> in analyze (not allowlisted)"
  cat /tmp/bc_str | while read -r line; do dim "  $line"; done
  VIOLATIONS=$((VIOLATIONS + 1))
else
  green "OK: No new FxHashSet<String> in analyze"
fi

echo ""
echo "── Transform ──"
TRANSFORM="$ROOT/crates/svelte_transform/src"

check "No OXC Parser in transform" \
  'oxc_parser::Parser|OxcParser::new' "$TRANSFORM" || true

check "No HashMap<String> in transform" \
  'HashMap<\s*String\b' "$TRANSFORM" || true

echo ""
echo "==================================="
if [[ "$VIOLATIONS" -gt 0 ]]; then
  red "$VIOLATIONS violation(s) found."
  exit 1
else
  green "All boundary checks passed."
  exit 0
fi
