#!/bin/sh
set -eu

docker run --rm --privileged --pid=host alpine sysctl -w kernel.perf_event_paranoid=1 >/dev/null 2>&1 || true

dir="$(dirname "$0")/local-mounts"
mkdir -p "$dir"

printf '#!/bin/sh\nexit 0\n' >"$dir/install.sh"
chmod +x "$dir/install.sh"

[ -f "$dir/devcontainer-feature.json" ] || cat >"$dir/devcontainer-feature.json" <<'EOF'
{
  "id": "local-mounts",
  "version": "1.0.0",
  "name": "Local bench corpora",
  "description": "Host directories to bench against. Gitignored. See ../local-mounts.example.json. Re-create the container after editing.",
  "mounts": []
}
EOF
