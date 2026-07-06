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

if [ -f package-lock.json ]; then
  npm ci
elif [ -f package.json ]; then
  npm install
fi

cat > ~/.vscode-ipc-refresh.sh <<'EOF'
# Keep VSCODE_IPC_HOOK_CLI pointing at a live VS Code IPC socket.
# Works around microsoft/vscode-remote-release#6997: the variable goes stale
# after reconnects/window reloads, which breaks `code --wait` as the git editor
# (rebase/commit fail with "Unable to connect to VS Code server").
__vscode_ipc_refresh() {
  case "${VSCODE_IPC_HOOK_CLI:-}" in
    /tmp/vscode-ipc-*)
      ss -lxH 2>/dev/null | grep -qF "$VSCODE_IPC_HOOK_CLI" && return ;;
  esac
  local live
  live=$(
    { ss -lxH 2>/dev/null | grep -o '/tmp/vscode-ipc-[^[:space:]]*\.sock' \
      || lsof -U 2>/dev/null | grep -o '/tmp/vscode-ipc-[^[:space:]]*\.sock'; } \
    | while IFS= read -r p; do
        [ -S "$p" ] && printf '%s\t%s\n' "$(stat -c %Y "$p" 2>/dev/null || echo 0)" "$p"
      done \
    | sort -rn | head -n1 | cut -f2-
  )
  [ -n "$live" ] && export VSCODE_IPC_HOOK_CLI="$live"
}
case "${PROMPT_COMMAND:-}" in
  *__vscode_ipc_refresh*) ;;
  *) PROMPT_COMMAND="__vscode_ipc_refresh${PROMPT_COMMAND:+; $PROMPT_COMMAND}" ;;
esac
EOF

if ! grep -q 'vscode-ipc-refresh.sh' ~/.bashrc 2>/dev/null; then
  echo '[ -f ~/.vscode-ipc-refresh.sh ] && source ~/.vscode-ipc-refresh.sh' >> ~/.bashrc
fi

# The workspace is a Docker Desktop bind mount (fakeowner/virtiofs) that fakes
# uid/gid/ctime/inode. git's default stat check then reports phantom changes,
# which makes interactive rebase fixup/squash abort with "Your local changes
# would be overwritten by merge". Comparing only mtime+size fixes it.
git config --global core.checkStat minimal

rustup show
cargo --version
just --version || true

echo "devcontainer ready"
