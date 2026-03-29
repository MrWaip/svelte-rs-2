#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <test-case-name>" >&2
  exit 1
fi

name="$1"

if [[ ! -d "tasks/compiler_tests/cases2/$name" ]]; then
  echo "Missing test case: tasks/compiler_tests/cases2/$name" >&2
  exit 1
fi

just test-case-verbose "$name"
