@echo off
setlocal

rem Path to the Rust workspace (contains Cargo.toml). The script lives in
rem scripts/, so the workspace is one level up.
set "PROJECT_DIR=%~dp0..\src-tauri"

pushd "%PROJECT_DIR%"
cargo test --lib %*
set "EXIT_CODE=%ERRORLEVEL%"
popd

exit /b %EXIT_CODE%