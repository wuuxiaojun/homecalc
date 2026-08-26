#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
OUT_DIR="$REPO_ROOT/web/src/lib/wasm"

mkdir -p "$OUT_DIR"

echo "Building engine-wasm for web target..."
wasm-pack build "$SCRIPT_DIR" \
    --target web \
    --out-dir "$OUT_DIR" \
    --out-name engine_wasm \
    --release

# Remove auto-generated .gitignore from wasm-pack so wasm assets can be tracked for Vercel/web deploys
rm -f "$OUT_DIR/.gitignore"

echo "WASM build complete. Artifacts written to $OUT_DIR"
