#!/usr/bin/env bash
# PreToolUse gate for the `required` skill.
# Once per session, before the first edit to compiler code (crates/**.rs),
# inject a non-blocking reminder to route the touched PRDs by their `topics:`
# line instead of judging docs by filename. Silent for any other file.
input=$(cat)
file=$(printf '%s' "$input" | jq -r '.tool_input.file_path // empty')
case "$file" in
  */crates/*.rs) ;;
  *) exit 0 ;;
esac
sid=$(printf '%s' "$input" | jq -r '.session_id // "nosession"')
sentinel="${TMPDIR:-/tmp}/claude-required-gate-${sid}"
if [ -e "$sentinel" ]; then exit 0; fi
touch "$sentinel" 2>/dev/null || true
msg='About to edit compiler code under crates/. First make sure the PRDs your task touches are in context: name your task terms in glossary vocabulary, then match them against each PRD `topics:` line (the `required` skill; docs/context.md + docs/map.md), never by filename. If you have not routed PRDs this session, do it now before editing.'
jq -cn --arg m "$msg" '{hookSpecificOutput:{hookEventName:"PreToolUse",additionalContext:$m}}'
