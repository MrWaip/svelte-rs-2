#!/usr/bin/env bash
set -euo pipefail

sudo chown -R vscode:vscode \
  /usr/local/cargo/registry \
  /usr/local/cargo/git \
  "$(pwd)/target" \
  "$(pwd)/node_modules" 2>/dev/null || true

if [ -f package.json ]; then
  npm install
fi

rustup show
cargo --version
just --version || true

echo "devcontainer ready"
