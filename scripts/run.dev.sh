#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# Script for running Tauri project with nightly and Cranelift
# Usage: run.dev.sh [--upd] [--clean] [--help]
# Note: nightly-only settings are passed via env vars
#       (RUSTUP_TOOLCHAIN, RUSTFLAGS), Cargo.toml stays stable-compatible.
# ============================================================

UPD=false
CLEAN=false
SHOW_HELP=false
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

# --- Ensure nightly toolchain is installed ---
echo "[INFO] Checking nightly toolchain"
if ! rustup which cargo --toolchain nightly >/dev/null 2>&1; then
    echo "[INFO] Nightly not installed. Installing..."
    if ! rustup toolchain install nightly; then
        echo "[ERROR] Failed to install nightly"
        exit 1
    fi
fi

if [ "$UPD" = true ]; then
    echo "[INFO] Running with dependency updates"

    # --- 1. Update nightly ---
    echo
    echo "=== Updating nightly Rust ==="
    if ! rustup update nightly; then
        echo "[ERROR] Failed to update nightly"
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
        exit 1
    fi
    echo "[OK] npm dependencies installed"
else
    echo
    echo "[WARN] Fast startup - updates skipped"
    echo "[INFO] Use --upd to update dependencies"
fi

# --- 4. Clean project in src-tauri (if --clean specified) ---
if [ "$CLEAN" = true ]; then
    echo
    echo "=== Cleaning project in src-tauri ==="
    (cd src-tauri && cargo +nightly clean) || echo "[WARN] Clean failed - continuing..."
    echo "[OK] Clean completed"
else
    echo
    echo "[WARN] Skipping project clean - use --clean to clean"
fi

# --- 5. Configure nightly via env (no default switch, no restore needed) ---
export RUSTUP_TOOLCHAIN="nightly"

RUSTFLAGS="-Z threads=8"

if [ "$CRANELIFT_INSTALLED" = true ]; then
    echo "[INFO] Using Cranelift for faster compilation"

    if rustup component list --toolchain nightly 2>/dev/null | grep -q "rustc-codegen-cranelift-preview.*installed"; then
        RUSTFLAGS="$RUSTFLAGS -Z codegen-backend=cranelift"
        echo "[INFO] Cranelift activated via RUSTFLAGS"
    elif command -v cargo-clif >/dev/null 2>&1; then
        echo "[INFO] Using cargo-clif instead of cargo"
        npm run tauri dev || { echo; echo "[ERROR] Failed to run Tauri"; exit 1; }
        exit 0
    else
        echo "[WARN] Cranelift not found, using standard backend"
    fi
else
    echo "[WARN] Cranelift not installed, using standard backend"
fi

export RUSTFLAGS="$RUSTFLAGS"
echo "[INFO] RUSTFLAGS=$RUSTFLAGS"
echo "[INFO] RUSTUP_TOOLCHAIN=nightly"

# --- 6. Run Tauri dev ---
echo
echo "=== Running Tauri dev with nightly + Cranelift ==="
echo "========================================="

if ! npm run tauri dev; then
    echo
    echo "[ERROR] Failed to run Tauri"
    exit 1
fi

echo