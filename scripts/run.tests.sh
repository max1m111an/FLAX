#!/usr/bin/env bash
set -euo pipefail

# Path to the Rust workspace (contains Cargo.toml). The script lives in
# scripts/, so the workspace is one level up.
PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)/src-tauri"

(
    cd "$PROJECT_DIR"
    cargo test --lib "$@"
)