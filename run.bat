@echo off
setlocal EnableExtensions

rem ============================================================
rem Script for running Tauri project FLAX
rem Usage: run.bat [--upd] [--help]
rem File should be saved in CP866 (OEM) encoding
rem ============================================================

set "UPD=false"
set "SHOW_HELP=false"

:parse_args
if "%~1"=="" goto :args_done
if /i "%~1"=="--upd" (
    set "UPD=true"
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
echo   Starting FLAX project
echo =========================================

rem --- Check for required tools ---
where npm >nul 2>nul || goto :no_npm
where rustup >nul 2>nul || goto :no_rustup

rem --- Check that we are in the project root ---
if not exist "package.json" (
    echo [ERROR] package.json not found! Run this script from the project root.
    exit /b 1
)
if not exist "src-tauri" (
    echo [ERROR] src-tauri folder not found! Run this script from the project root.
    exit /b 1
)

if "%UPD%"=="false" (
    echo.
    echo [WARN] Fast startup - updates skipped
    echo [INFO] Use --upd to update dependencies
    goto :run_project
)

echo [INFO] Running with dependency updates

rem --- Update Rust ---
echo.
echo [INFO] Updating Rust (rustup update)...
rustup update
if errorlevel 1 goto :upd_failed
echo [OK] Rust successfully updated

rem --- Install npm dependencies ---
echo.
echo [INFO] Installing npm dependencies...
call npm install
if errorlevel 1 goto :npm_failed
echo [OK] npm dependencies installed

:run_project
rem --- Run the project ---
echo.
echo [INFO] Starting Tauri in development mode...
echo =========================================
call npm run tauri dev
if errorlevel 1 goto :dev_failed
echo.
exit /b 0

:show_help
echo Usage: run.bat [OPTIONS]
echo.
echo Options:
echo   --upd       Update Rust and run npm install before starting
echo   --help, -h  Show this help message
echo.
echo Examples:
echo   run.bat           Quick start without updates
echo   run.bat --upd     Start with dependency updates
exit /b 0

:no_npm
echo [ERROR] npm not found! Install Node.js
exit /b 1

:no_rustup
echo [ERROR] rustup not found! Install Rust
exit /b 1

:upd_failed
echo [ERROR] Failed to update Rust
exit /b 1

:npm_failed
echo [ERROR] Failed to install npm dependencies
exit /b 1

:dev_failed
echo.
echo [ERROR] Failed to start the project
exit /b 1