#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# Script for running Tauri project with nightly and Cranelift
# Usage: run.dev.sh [--upd] [--clean] [--help]
# ============================================================

UPD=false
CLEAN=false
SHOW_HELP=false
ORIGINAL_TOOLCHAIN=""
CRANELIFT_INSTALLED=false

usage() {
    echo "Usage: run.dev.sh [OPTIONS]"
    echo
    echo "Options:"
    echo "  --upd       Update nightly Rust, install Cranelift and run npm install"
    echo "  --clean     Clean project (cargo clean) before building"
    echo "  --help, -h  Show this help message"
    echo
    echo "Examples:"
    echo "  run.dev.sh                Quick start with nightly"
    echo "  run.dev.sh --upd          Start with dependency updates and Cranelift install"
    echo "  run.dev.sh --clean        Start with project clean"
    echo "  run.dev.sh --upd --clean  Full start with updates and clean"
    echo
    echo "Note: Script automatically tries to install Cranelift if not found."
    exit 0
}

restore_toolchain() {
    if [ -n "$ORIGINAL_TOOLCHAIN" ]; then
        echo "[INFO] Restoring original toolchain: $ORIGINAL_TOOLCHAIN"
        rustup default "$ORIGINAL_TOOLCHAIN" >/dev/null 2>&1 || true
    fi
}

for arg in "$@"; do
    case "$arg" in
        --upd) UPD=true ;;
        --clean) CLEAN=true ;;
        --help|-h) SHOW_HELP=true ;;
        *)
            echo "[ERROR] Unknown argument: $arg"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

if [ "$SHOW_HELP" = true ]; then
    usage
fi

echo "========================================="
echo "  Running Tauri FLAX with nightly + Cranelift"
echo "========================================="

# --- Check for required tools ---
if ! command -v rustup >/dev/null 2>&1; then
    echo "[ERROR] rustup not found! Install Rust: https://rustup.rs/"
    exit 1
fi
if ! command -v npm >/dev/null 2>&1; then
    echo "[ERROR] npm not found! Install Node.js: https://nodejs.org/"
    exit 1
fi

# --- Check that we are in the project root ---
if [ ! -d "src-tauri" ]; then
    echo "[ERROR] src-tauri folder not found! Run this script from the project root."
    exit 1
fi
if [ ! -f "package.json" ]; then
    echo "[ERROR] package.json not found! Run this script from the project root."
    exit 1
fi

# --- Save current toolchain for restoration ---
ORIGINAL_TOOLCHAIN="$(rustup default 2>/dev/null | awk '{print $1}')"
echo "[INFO] Current toolchain: $ORIGINAL_TOOLCHAIN"

if [ "$UPD" = false ]; then
    echo
    echo "[WARN] Fast startup - updates skipped"
    echo "[INFO] Use --upd to update dependencies"
else
    echo "[INFO] Running with dependency updates"

    # --- 1. Update nightly ---
    echo
    echo "=== Updating nightly Rust ==="
    if ! rustup update nightly; then
        echo "[ERROR] Failed to update nightly"
        restore_toolchain
        exit 1
    fi
    echo "[OK] Nightly successfully updated"

    # --- 2. Check and install Cranelift component ---
    echo
    echo "=== Checking Cranelift component ==="

    if rustup component list --toolchain nightly 2>/dev/null | grep -q "rustc-codegen-cranelift-preview.*installed"; then
        echo "[OK] Cranelift component already installed"
        CRANELIFT_INSTALLED=true
    else
        echo "[INFO] Cranelift component not installed. Trying to install..."

        if rustup component add rustc-codegen-cranelift-preview --toolchain nightly 2>/dev/null; then
            echo "[OK] Cranelift component successfully installed"
            CRANELIFT_INSTALLED=true
        else
            echo "[WARN] Failed to install Cranelift component via rustup"
            echo "[WARN] Check availability: https://github.com/rust-lang/rustc_codegen_cranelift"
            echo "[WARN] Continuing without Cranelift..."
            CRANELIFT_INSTALLED=false
        fi
    fi

    # --- Alternative installation via cargo (if rustup failed) ---
    if [ "$CRANELIFT_INSTALLED" = false ]; then
        echo
        echo "[INFO] Trying to install Cranelift via cargo..."

        if ! command -v cargo-clif >/dev/null 2>&1; then
            echo "[INFO] Installing cargo-clif..."
            if cargo install cargo-clif 2>/dev/null; then
                echo "[OK] cargo-clif installed"
                CRANELIFT_INSTALLED=true
            else
                echo "[WARN] Failed to install cargo-clif"
                echo "[WARN] Continuing without Cranelift"
            fi
        else
            echo "[OK] cargo-clif already installed"
            CRANELIFT_INSTALLED=true
        fi
    fi

    # --- 3. Install npm dependencies ---
    echo
    echo "=== Installing npm dependencies ==="
    if ! npm install; then
        echo "[ERROR] Failed to install npm dependencies"
        restore_toolchain
        exit 1
    fi
    echo "[OK] npm dependencies installed"
fi

# --- 4. Clean project in src-tauri (if --clean specified) ---
if [ "$CLEAN" = false ]; then
    echo
    echo "[WARN] Skipping project clean - use --clean to clean"
else
    echo
    echo "=== Cleaning project in src-tauri ==="
    (cd src-tauri && cargo +nightly clean) || echo "[WARN] Clean failed - continuing..."
    echo "[OK] Clean completed"
fi

# --- 5. Set nightly as default toolchain ---
echo
echo "=== Setting nightly as default toolchain ==="
if ! rustup default nightly; then
    echo "[ERROR] Failed to set nightly as default"
    restore_toolchain
    exit 1
fi
echo "[OK] Nightly set as default toolchain"

# --- 6. Run Tauri dev ---
echo
echo "=== Running Tauri dev with nightly + Cranelift ==="
echo "========================================="

RUSTFLAGS=""

if [ "$CRANELIFT_INSTALLED" = true ]; then
    echo "[INFO] Using Cranelift for faster compilation"

    if rustup component list --toolchain nightly 2>/dev/null | grep -q "rustc-codegen-cranelift-preview.*installed"; then
        RUSTFLAGS="-Z codegen-backend=cranelift"
        echo "[INFO] Cranelift activated via RUSTFLAGS"
    elif command -v cargo-clif >/dev/null 2>&1; then
        echo "[INFO] Using cargo-clif instead of cargo"
        cargo-clif tauri dev || { echo; echo "[ERROR] Failed to run Tauri"; restore_toolchain; exit 1; }
        restore_toolchain
        exit 0
    else
        echo "[WARN] Cranelift not found, using standard backend"
    fi
else
    echo "[WARN] Cranelift not installed, using standard backend"
fi

if [ -n "$RUSTFLAGS" ]; then
    echo "[INFO] RUSTFLAGS=$RUSTFLAGS"
    export RUSTFLAGS="$RUSTFLAGS"
else
    echo "[INFO] Using standard build without Cranelift"
fi

if ! npm run tauri dev; then
    echo
    echo "[ERROR] Failed to run Tauri"
    restore_toolchain
    exit 1
fi

echo
restore_toolchain
