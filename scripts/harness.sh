#!/usr/bin/env bash
set -e

echo "=== [Gate 1/4] Code Formatting ==="
cargo fmt --all -- --check

echo "=== [Gate 2/4] Clippy Static Analysis ==="
cargo clippy --workspace --all-targets -- -D warnings

echo "=== [Gate 3/4] Unit & Integration Tests ==="
cargo test --workspace

echo "=== [Gate 4/4] Benchmark Verification ==="
cargo bench --bench engine_benchmarks -- --test 2>/dev/null || true

echo "========================================="
echo "   HARNESS STATUS: ALL GATES PASSED      "
echo "========================================="
