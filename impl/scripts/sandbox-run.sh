#!/usr/bin/env bash
# Build and run the VyroLang stack (Compiler API + VyroIDE + VyroVM) inside a
# resource-limited Docker container — the "Docker Sandbox" layer of the diagram.
set -euo pipefail
cd "$(dirname "$0")/.."

if ! command -v docker >/dev/null 2>&1; then
  echo "Docker not found. Run locally instead:  cargo run --release -- serve 8787"
  exit 1
fi

echo "› Building the sandbox image…"
docker build -t vyro-ide .

echo "› Running with CPU/RAM/PID limits, read-only fs, dropped capabilities…"
exec docker run --rm -it \
  -p 8787:8787 \
  --memory=256m --memory-swap=256m \
  --pids-limit=256 \
  --cpus=1 \
  --read-only --tmpfs /tmp \
  --cap-drop=ALL --security-opt no-new-privileges \
  vyro-ide
