#!/usr/bin/env bash
# PreToolUse hook: blocks Edit/Write that introduce architectural violations.
# Checks the FULL resulting file, not just the diff — existing violations
# are allowlisted explicitly, so they cannot be replicated or spread.
#
# Receives JSON on stdin: { "tool_name": "Edit"|"Write", "tool_input": { ... } }
# Exit 0 = allow, Exit 2 = block.
set -euo pipefail

INPUT=$(cat)
TOOL=$(echo "$INPUT" | jq -r '.tool_name')
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Only check Rust source files in our crates
[[ "$FILE_PATH" == */crates/*.rs ]] || exit 0

# ── Build the resulting file content ──
if [[ "$TOOL" == "Write" ]]; then
  FULL_CONTENT=$(echo "$INPUT" | jq -r '.tool_input.content // empty')
elif [[ "$TOOL" == "Edit" ]]; then
  # For Edit, we need to simulate the replacement on the current file
  OLD=$(echo "$INPUT" | jq -r '.tool_input.old_string // empty')
  NEW=$(echo "$INPUT" | jq -r '.tool_input.new_string // empty')
  if [[ -f "$FILE_PATH" ]]; then
    CURRENT=$(<"$FILE_PATH")
    # Replace first occurrence of old with new (matching Edit semantics)
    FULL_CONTENT="${CURRENT/"$OLD"/"$NEW"}"
  else
    # New file via Edit — just check the new string
    FULL_CONTENT="$NEW"
  fi
else
  exit 0
fi

[[ -n "$FULL_CONTENT" ]] || exit 0

# ── Determine crate ──
CRATE=""
FILE_REL="${FILE_PATH##*/crates/}"
case "$FILE_PATH" in
  *svelte_codegen_client*) CRATE="codegen" ;;
  *svelte_transform*)      CRATE="transform" ;;
  *svelte_analyze*)        CRATE="analyze" ;;
  *svelte_parser*)         CRATE="parser" ;;
  *svelte_ast*)            CRATE="ast" ;;
  *)                       exit 0 ;;
esac

# ── Allowlist ──
# Each entry: "file_basename:pattern" — exact known violations that exist today.
# These are NOT license to create new ones — only to keep the hook from blocking
# edits to files that already contain them.
ALLOWLIST=(
  # builder.rs and script/mod.rs legitimately use OXC parser in codegen
  "builder.rs:oxc_parser"
  "mod.rs:oxc_parser"  # script/mod.rs
  # analyze: rest_prop uses FxHashSet<String> (needs migration to SymbolId)
  "scope.rs:FxHashSet<String>"
  "ident_gen.rs:FxHashSet<String>"
  # analyze: script_info.rs has shallow Expression:: matching for rune detection
  "script_info.rs:Expression::"
  # analyze: ce_config.rs has shallow Expression:: matching for config parsing
  "ce_config.rs:Expression::"
  # analyze: template_side_tables.rs has one Expression::Identifier check
  "template_side_tables.rs:Expression::"
)

is_allowlisted() {
  local file_base pattern
  file_base=$(basename "$FILE_PATH")
  pattern="$1"
  for entry in "${ALLOWLIST[@]}"; do
    local al_file="${entry%%:*}"
    local al_pattern="${entry#*:}"
    if [[ "$file_base" == "$al_file" && "$pattern" == "$al_pattern" ]]; then
      return 0
    fi
  done
  return 1
}

# Strip lines with BOUNDARY-OK comments
CHECKED=$(echo "$FULL_CONTENT" | grep -v '// BOUNDARY-OK' || true)

VIOLATIONS=""

# ── Rule 1: No HashMap<String> outside codegen ──
if [[ "$CRATE" != "codegen" ]]; then
  if echo "$CHECKED" | grep -qP 'HashMap<\s*String\b' 2>/dev/null; then
    VIOLATIONS+="BOUNDARY: HashMap<String> in $CRATE ($FILE_REL). Use FxHashMap with SymbolId/NodeId keys.\n"
  fi
fi

# ── Rule 2: No FxHashSet<String> (except allowlisted) ──
if echo "$CHECKED" | grep -qP 'FxHashSet<String' 2>/dev/null; then
  if ! is_allowlisted "FxHashSet<String>"; then
    VIOLATIONS+="BOUNDARY: FxHashSet<String> in $CRATE ($FILE_REL). Use SymbolId-based lookups.\n"
  fi
fi

# ── Rule 3: No OXC parser in codegen (except allowlisted) ──
if [[ "$CRATE" == "codegen" ]]; then
  if echo "$CHECKED" | grep -qP 'oxc_parser::Parser|OxcParser::new' 2>/dev/null; then
    if ! is_allowlisted "oxc_parser"; then
      VIOLATIONS+="BOUNDARY: OXC Parser in codegen ($FILE_REL). Parsing belongs in svelte_parser.\n"
    fi
  fi
fi

# ── Rule 4: No string parsing in codegen ──
if [[ "$CRATE" == "codegen" ]]; then
  if echo "$CHECKED" | grep -qP "starts_with\s*\(\s*['\"][{\[('\"]" 2>/dev/null; then
    VIOLATIONS+="BOUNDARY: String parsing heuristic (starts_with '{'/'{'/etc.) in codegen ($FILE_REL). Use structured AST.\n"
  fi
  if echo "$CHECKED" | grep -qP "split\s*\(\s*['\"][,:=]['\"]" 2>/dev/null; then
    VIOLATIONS+="BOUNDARY: String splitting in codegen ($FILE_REL). Structure in parser/analyze.\n"
  fi
fi

# ── Rule 5: No Expression:: in codegen template ──
if [[ "$CRATE" == "codegen" ]]; then
  case "$FILE_PATH" in
    *builder.rs | *script/*) ;;
    *)
      if echo "$CHECKED" | grep -qP 'Expression::\w+' 2>/dev/null; then
        VIOLATIONS+="BOUNDARY: Expression:: match in codegen template ($FILE_REL). Move classification to analyze.\n"
      fi
      ;;
  esac
fi

# ── Rule 6: No Statement:: in codegen template ──
if [[ "$CRATE" == "codegen" ]]; then
  case "$FILE_PATH" in
    *builder.rs | *script/*) ;;
    *)
      if echo "$CHECKED" | grep -qP 'Statement::\w+' 2>/dev/null; then
        VIOLATIONS+="BOUNDARY: Statement:: match in codegen template ($FILE_REL). Codegen shouldn't inspect JS AST.\n"
      fi
      ;;
  esac
fi

# ── Rule 7: Broad Expression:: traversal in analyze (5+ variants = manual walk) ──
if [[ "$CRATE" == "analyze" ]]; then
  if ! is_allowlisted "Expression::"; then
    EXPR_COUNT=$(echo "$CHECKED" | grep -cP 'Expression::\w+' 2>/dev/null || true)
    if [[ "$EXPR_COUNT" -ge 5 ]]; then
      VIOLATIONS+="BOUNDARY: $EXPR_COUNT Expression:: variants in analyze ($FILE_REL). Use OXC Visit, not manual matching.\n"
    fi
  fi
fi

# ── Rule 8: Deep AnalysisData chaining in codegen ──
if [[ "$CRATE" == "codegen" ]]; then
  if echo "$CHECKED" | grep -qP 'ctx\.analysis\.\w+\([^)]*\)\.\w+\([^)]*\)\.\w+' 2>/dev/null; then
    VIOLATIONS+="BOUNDARY: Deep AnalysisData chaining in codegen ($FILE_REL). Add accessor to AnalysisData or Ctx shortcut.\n"
  fi
fi

# ── Rule 9: No OXC parser in transform ──
if [[ "$CRATE" == "transform" ]]; then
  if echo "$CHECKED" | grep -qP 'oxc_parser::Parser|OxcParser::new' 2>/dev/null; then
    VIOLATIONS+="BOUNDARY: OXC Parser in transform ($FILE_REL). Parsing belongs in svelte_parser.\n"
  fi
fi

if [[ -n "$VIOLATIONS" ]]; then
  echo -e "Architecture boundary violation(s) blocked:\n"
  echo -e "$VIOLATIONS"
  echo "Existing violations are NOT precedent — do not replicate them."
  echo "Fix: move logic to correct phase. See CLAUDE.md 'Architecture boundaries'."
  echo "Legitimate exception: add '// BOUNDARY-OK: <reason>' on that line."
  exit 2
fi

exit 0
