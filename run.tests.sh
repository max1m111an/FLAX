#!/usr/bin/env bash
set -euo pipefail

# Path to the Rust workspace (contains Cargo.toml).
PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)/src-tauri"

(
    cd "$PROJECT_DIR"
    cargo test --lib "$@"
)