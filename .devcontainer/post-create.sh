#!/usr/bin/env bash
set -euo pipefail

sudo chown -R vscode:vscode \
  /usr/local/cargo/registry \
  /usr/local/cargo/git \
  "$(pwd)/target" \
  "$(pwd)/node_modules" 2>/dev/null || true

mkdir -p ~/.ssh
chmod 700 ~/.ssh
if ! grep -q '^Host github.com$' ~/.ssh/config 2>/dev/null; then
  cat >> ~/.ssh/config <<'EOF'

Host github.com
  Hostname ssh.github.com
  Port 443
  User git
  ServerAliveInterval 30
  StrictHostKeyChecking accept-new
EOF
  chmod 600 ~/.ssh/config
fi

if [ -f package.json ]; then
  npm install
fi

rustup show
cargo --version
just --version || true

echo "devcontainer ready"
