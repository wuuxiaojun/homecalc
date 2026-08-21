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

echo "WASM build complete. Artifacts written to $OUT_DIR"
