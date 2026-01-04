#!/usr/bin/env bash
set -euo pipefail

# Ctrl+C で子プロセス全部を終了させる
trap 'kill 0' INT TERM EXIT

# 設定を読み込み
ARTICLES_DIR=$(python3 scripts/config.py articles_dir --relative)
APP_DIR=$(python3 scripts/config.py app_dir --relative)
DEBOUNCE_MS=$(python3 scripts/config.py debounce_ms 2>/dev/null || echo "300")

# 依存チェック
command -v watchexec >/dev/null || { echo "watchexec not found. Install it (nix profile install nixpkgs#watchexec)"; exit 1; }
command -v trunk >/dev/null || { echo "trunk not found. Install trunk (https://trunkrs.dev/)"; exit 1; }
command -v just >/dev/null || { echo "just not found. Install just"; exit 1; }

echo "🚀 Dev environment starting..."
echo "  - Watching articles: ${ARTICLES_DIR}"
echo ""

# 初回ビルド（データ生成のみ）
just dev-data-only

# Watcher 1: Articles -> Data Pipeline
watchexec \
  -w "${ARTICLES_DIR}" \
  -e md \
  --restart \
  --delay-run "${DEBOUNCE_MS}ms" \
  -- just _on-article-change &

# Watcherの起動待ち
sleep 0.5

# Start trunk serve
cd "${APP_DIR}"
echo "Starting trunk serve (debug) — open your browser at the printed URL"
trunk serve --open
