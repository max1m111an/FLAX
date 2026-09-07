#!/usr/bin/env bash
set -euo pipefail

# Path to the Rust workspace (contains Cargo.toml and .cargo/config.toml).
PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)/src-tauri"
# File with LLVM override for the dev profile (avoids the flaky cranelift backend).
CFG="$PROJECT_DIR/.cargo/config.toml"

# Ensure a nightly toolchain is used (required by cargo-features in Cargo.toml).
export RUSTUP_TOOLCHAIN="nightly"

EXIT_CODE=0
CFG_CREATED=false

if [ ! -f "$CFG" ]; then
    mkdir -p "$PROJECT_DIR/.cargo" 2>/dev/null
    printf '[profile.dev]\ncodegen-backend = "llvm"\n' > "$CFG"
    CFG_CREATED=true
fi

(
    cd "$PROJECT_DIR"
    cargo test --lib "$@"
)
EXIT_CODE=$?

if [ "$CFG_CREATED" = true ]; then
    rm -f "$CFG" 2>/dev/null || true
    rmdir "$PROJECT_DIR/.cargo" 2>/dev/null || true
fi

exit $EXIT_CODE
