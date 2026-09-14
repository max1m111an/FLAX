@echo off
setlocal EnableExtensions

rem ============================================================
rem Script for running Tauri project with nightly and Cranelift
rem Usage: run.dev.bat [--upd] [--clean] [--help]
rem File should be saved in CP866 (OEM) encoding
rem Note: nightly-only settings are passed via env vars
rem       (RUSTUP_TOOLCHAIN, RUSTFLAGS), Cargo.toml stays stable-compatible.
rem ============================================================

set "UPD=false"
set "CLEAN=false"
set "SHOW_HELP=false"
set "CRANELIFT_INSTALLED=false"

:parse_args
if "%~1"=="" goto :args_done
if /i "%~1"=="--upd" (
    set "UPD=true"
    shift
    goto :parse_args
)
if /i "%~1"=="--clean" (
    set "CLEAN=true"
    shift
    goto :parse_args
)
if /i "%~1"=="--help" (
    set "SHOW_HELP=true"
    shift
    goto :parse_args
)
if /i "%~1"=="-h" (
    set "SHOW_HELP=true"
    shift
    goto :parse_args
)
echo [ERROR] Unknown argument: %~1
echo Use --help for usage information
exit /b 1

:args_done
if "%SHOW_HELP%"=="true" goto :show_help

echo =========================================
echo   Running Tauri FLAX with nightly + Cranelift
echo =========================================

rem --- Check for required tools ---
where rustup >nul 2>nul || goto :no_rustup
where npm >nul 2>nul || goto :no_npm

rem --- Check that we are in the project root ---
if not exist "src-tauri" (
    echo [ERROR] src-tauri folder not found! Run this script from the project root.
    exit /b 1
)
if not exist "package.json" (
    echo [ERROR] package.json not found! Run this script from the project root.
    exit /b 1
)

rem --- Ensure nightly toolchain is installed ---
echo [INFO] Checking nightly toolchain
rustup which cargo --toolchain nightly >nul 2>nul
if errorlevel 1 (
    echo [INFO] Nightly not installed. Installing...
    rustup toolchain install nightly
    if errorlevel 1 goto :err_install_nightly
)

if "%UPD%"=="false" (
    echo.
    echo [WARN] Fast startup - updates skipped
    echo [INFO] Use --upd to update dependencies
    goto :maybe_clean
)

echo [INFO] Running with dependency updates

rem --- 1. Update nightly ---
echo.
echo === Updating nightly Rust ===
rustup update nightly
if errorlevel 1 goto :err_upd_nightly
echo [OK] Nightly successfully updated

rem --- 2. Check and install Cranelift component ---
echo.
echo === Checking Cranelift component ===

rem Check if Cranelift is already installed
rustup component list --toolchain nightly 2>nul | findstr /R /C:"rustc-codegen-cranelift-preview.*installed" >nul
if errorlevel 1 (
    echo [INFO] Cranelift component not installed. Trying to install...

    rem Try to install Cranelift via rustup
    rustup component add rustc-codegen-cranelift-preview --toolchain nightly 2>nul
    if errorlevel 1 (
        echo [WARN] Failed to install Cranelift component via rustup
        echo [WARN] Check availability: https://github.com/rust-lang/rustc_codegen_cranelift
        echo [WARN] Continuing without Cranelift...
        set "CRANELIFT_INSTALLED=false"
    ) else (
        echo [OK] Cranelift component successfully installed
        set "CRANELIFT_INSTALLED=true"
    )
) else (
    echo [OK] Cranelift component already installed
    set "CRANELIFT_INSTALLED=true"
)

rem --- Alternative installation via cargo (if rustup failed) ---
if "%CRANELIFT_INSTALLED%"=="false" (
    echo.
    echo [INFO] Trying to install Cranelift via cargo...

    rem Check if cargo-clif is available
    where cargo-clif >nul 2>nul
    if errorlevel 1 (
        echo [INFO] Installing cargo-clif...
        cargo install cargo-clif 2>nul
        if errorlevel 1 (
            echo [WARN] Failed to install cargo-clif
            echo [WARN] Continuing without Cranelift
        ) else (
            echo [OK] cargo-clif installed
            set "CRANELIFT_INSTALLED=true"
        )
    ) else (
        echo [OK] cargo-clif already installed
        set "CRANELIFT_INSTALLED=true"
    )
)

rem --- 3. Install npm dependencies ---
echo.
echo === Installing npm dependencies ===
call npm install
if errorlevel 1 goto :err_npm_install
echo [OK] npm dependencies installed

:maybe_clean
rem --- 4. Clean project in src-tauri (if --clean specified) ---
if "%CLEAN%"=="false" (
    echo.
    echo [WARN] Skipping project clean - use --clean to clean
    goto :config_env
)

echo.
echo === Cleaning project in src-tauri ===
pushd src-tauri || goto :err_cd_tauri
cargo +nightly clean
if errorlevel 1 echo [WARN] Clean failed - continuing...
popd
echo [OK] Clean completed

:config_env
rem --- 5. Configure nightly via env (no default switch) ---
set "RUSTUP_TOOLCHAIN=nightly"
set "RUSTFLAGS=-Z threads=8"

if "%CRANELIFT_INSTALLED%"=="true" (
    echo [INFO] Using Cranelift for faster compilation

    rem Check that component is actually installed via rustup
    rustup component list --toolchain nightly 2>nul | findstr /R /C:"rustc-codegen-cranelift-preview.*installed" >nul
    if not errorlevel 1 (
        set "RUSTFLAGS=%RUSTFLAGS% -Z codegen-backend=cranelift"
        echo [INFO] Cranelift activated via RUSTFLAGS
    ) else (
        rem Check for cargo-clif
        where cargo-clif >nul 2>nul
        if not errorlevel 1 (
            echo [INFO] Using cargo-clif instead of cargo
            call npm run tauri dev
            if errorlevel 1 goto :err_dev
            echo.
            exit /b 0
        ) else (
            echo [WARN] Cranelift not found, using standard backend
        )
    )
) else (
    echo [WARN] Cranelift not installed, using standard backend
)

echo [INFO] RUSTFLAGS=%RUSTFLAGS%
echo [INFO] RUSTUP_TOOLCHAIN=nightly

rem --- 6. Run Tauri dev ---
echo.
echo === Running Tauri dev with nightly + Cranelift ===
echo =========================================

call npm run tauri dev
if errorlevel 1 goto :err_dev
echo.
exit /b 0

:show_help
echo Usage: run.dev.bat [OPTIONS]
echo.
echo Options:
echo   --upd       Update nightly Rust, install Cranelift and run npm install
echo   --clean     Clean project (cargo clean) before building
echo   --help, -h  Show this help message
echo.
echo Examples:
echo   run.dev.bat                Quick start with nightly
echo   run.dev.bat --upd          Start with dependency updates and Cranelift install
echo   run.dev.bat --clean        Start with project clean
echo   run.dev.bat --upd --clean  Full start with updates and clean
echo.
echo Note: Script automatically tries to install Cranelift if not found.
exit /b 0

:no_rustup
echo [ERROR] rustup not found! Install Rust: https://rustup.rs/
exit /b 1

:no_npm
echo [ERROR] npm not found! Install Node.js: https://nodejs.org/
exit /b 1

:err_install_nightly
echo [ERROR] Failed to install nightly
exit /b 1

:err_upd_nightly
echo [ERROR] Failed to update nightly
exit /b 1

:err_npm_install
echo [ERROR] Failed to install npm dependencies
exit /b 1

:err_cd_tauri
echo [ERROR] Failed to change to src-tauri folder
exit /b 1

:err_dev
echo.
echo [ERROR] Failed to run Tauri
exit /b 1