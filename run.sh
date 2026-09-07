#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# Script for running Tauri project FLAX
# Usage: run.sh [--upd] [--help]
# ============================================================

UPD=false
SHOW_HELP=false

usage() {
    echo "Usage: run.sh [OPTIONS]"
    echo
    echo "Options:"
    echo "  --upd       Update Rust and run npm install before starting"
    echo "  --help, -h  Show this help message"
    echo
    echo "Examples:"
    echo "  run.sh           Quick start without updates"
    echo "  run.sh --upd     Start with dependency updates"
    exit 0
}

for arg in "$@"; do
    case "$arg" in
        --upd) UPD=true ;;
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
echo "  Starting FLAX project"
echo "========================================="

# --- Check for required tools ---
if ! command -v npm >/dev/null 2>&1; then
    echo "[ERROR] npm not found! Install Node.js"
    exit 1
fi
if ! command -v rustup >/dev/null 2>&1; then
    echo "[ERROR] rustup not found! Install Rust"
    exit 1
fi

# --- Check that we are in the project root ---
if [ ! -f "package.json" ]; then
    echo "[ERROR] package.json not found! Run this script from the project root."
    exit 1
fi
if [ ! -d "src-tauri" ]; then
    echo "[ERROR] src-tauri folder not found! Run this script from the project root."
    exit 1
fi

if [ "$UPD" = false ]; then
    echo
    echo "[WARN] Fast startup - updates skipped"
    echo "[INFO] Use --upd to update dependencies"
else
    echo "[INFO] Running with dependency updates"

    # --- Update Rust ---
    echo
    echo "[INFO] Updating Rust (rustup update)..."
    if ! rustup update; then
        echo "[ERROR] Failed to update Rust"
        exit 1
    fi
    echo "[OK] Rust successfully updated"

    # --- Install npm dependencies ---
    echo
    echo "[INFO] Installing npm dependencies..."
    if ! npm install; then
        echo "[ERROR] Failed to install npm dependencies"
        exit 1
    fi
    echo "[OK] npm dependencies installed"
fi

# --- Run the project ---
echo
echo "[INFO] Starting Tauri in development mode..."
echo "========================================="
if ! npm run tauri dev; then
    echo
    echo "[ERROR] Failed to start the project"
    exit 1
fi
echo
