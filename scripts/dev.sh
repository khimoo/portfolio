#!/usr/bin/env bash
set -euo pipefail

# Kill all child processes on exit
trap 'kill 0' INT TERM EXIT

# Load configuration
ARTICLES_DIR=$(python3 scripts/config.py articles_dir --relative)
APP_DIR=$(python3 scripts/config.py app_dir --relative)
DEBOUNCE_MS=$(python3 scripts/config.py debounce_ms --section build 2>/dev/null || echo "300")

# Check dependencies
for cmd in watchexec trunk just; do
    command -v "$cmd" >/dev/null || { echo "Error: $cmd not found"; exit 1; }
done

echo "🚀 Starting development environment..."
echo "  Watching: ${ARTICLES_DIR}"

# Initial data build
just process-data

# Start file watcher for articles
watchexec \
  -w "${ARTICLES_DIR}" \
  -e md \
  --restart \
  --delay-run "${DEBOUNCE_MS}ms" \
  -- just _on-article-change &

# Start trunk serve
cd "${APP_DIR}"
if [ "${GITHUB_PAGES_MODE:-}" = "1" ]; then
    PUBLIC_URL=$(python3 ../scripts/config.py github_pages_path --section deployment)
    echo "Starting with GitHub Pages path: http://127.0.0.1:8080${PUBLIC_URL}"
    trunk serve --public-url "${PUBLIC_URL}" --open
else
    PUBLIC_URL=$(python3 ../scripts/config.py local_dev_path --section deployment)
    echo "Starting local development: http://127.0.0.1:8080${PUBLIC_URL}"
    trunk serve --public-url "${PUBLIC_URL}" --open
fi
