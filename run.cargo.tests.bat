@echo off
setlocal

rem Path to the Rust workspace (contains Cargo.toml).
set "PROJECT_DIR=%~dp0src-tauri"

pushd "%PROJECT_DIR%"
cargo test --lib %*
set "EXIT_CODE=%ERRORLEVEL%"
popd

exit /b %EXIT_CODE%