#!/usr/bin/env bash
set -euo pipefail

scope="${1:-}"

if [[ -n "$scope" ]]; then
  echo "[qa] Reviewing diff: $scope..HEAD"
  git diff --name-only "$scope"..HEAD
else
  if [[ -n "$(git diff --name-only HEAD)" ]]; then
    echo "[qa] Reviewing uncommitted changes vs HEAD"
    git diff --name-only HEAD
  else
    echo "[qa] Reviewing last commit (HEAD~1..HEAD)"
    git diff --name-only HEAD~1..HEAD
  fi
fi

echo

echo "[qa] Recommended verification commands (run as needed):"
echo "  just test-parser"
echo "  just test-analyzer"
echo "  just test-compiler"
echo "  just test-all"
