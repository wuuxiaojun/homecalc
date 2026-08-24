#!/usr/bin/env bash
set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_ROOT/web"

# Auto-build production dist if missing
if [ ! -d "dist" ]; then
    echo "Building Homecalc v2.0.0 production bundle..."
    npm run build
fi

# Serve the compiled production build and automatically open the browser
npx vite preview --open --port 5173
