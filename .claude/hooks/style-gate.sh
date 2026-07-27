#!/usr/bin/env bash
# PreToolUse gate for the `rust-style` and `write-unit-test` skills.
# `paths:` in a SKILL.md only restricts activation, it never triggers one, so a
# standard that must hold on every edit is enforced here instead. Each pointer
# fires once per session: rust-style before the first .rs edit, write-unit-test
# before the first edit that introduces a `#[test]` or an `assert_*` under
# crates/. Silent for any other file.
input=$(cat)
file=$(printf '%s' "$input" | jq -r '.tool_input.file_path // empty')
case "$file" in
  *.rs) ;;
  *) exit 0 ;;
esac

sid=$(printf '%s' "$input" | jq -r '.session_id // "nosession"')
tmp="${TMPDIR:-/tmp}"
out=""

fire() {
  sentinel="$tmp/claude-style-gate-$1-${sid}"
  if [ -e "$sentinel" ]; then return 1; fi
  touch "$sentinel" 2>/dev/null || true
  return 0
}

if fire style; then
  out='About to edit Rust. The `rust-style` skill is non-negotiable and applies to this edit: guard clauses with the happy path unindented, `Result` and `?` instead of `panic!`/`unwrap`/`expect`/`unreachable!`/`todo!` in production code, exhaustive `match` over domain enums you own, no negated condition carrying an `else`, readable loops over crammed iterator chains, and no comments. Read the skill now if it is not already in context.'
fi

case "$file" in
  */crates/*/src/*)
    body=$(printf '%s' "$input" | jq -r '[.tool_input.content, .tool_input.new_string, (.tool_input.edits // [] | map(.new_string) | join("\n"))] | map(select(. != null)) | join("\n")')
    case "$body" in
      *'#[test]'*|*assert_*)
        if fire unittest; then
          msg='This edit adds a unit test or an assertion helper. The `write-unit-test` format applies: a body of at most three lines — a Creation Method for setup, an optional execute line, and exactly one `#[track_caller]` custom `assert_*`. Never stack several asserts in a test body; bundle them inside one assertion instead. Read the skill now if it is not already in context.'
          out="${out:+$out }$msg"
        fi
        ;;
    esac
    ;;
esac

if [ -z "$out" ]; then exit 0; fi
jq -cn --arg m "$out" '{hookSpecificOutput:{hookEventName:"PreToolUse",additionalContext:$m}}'
