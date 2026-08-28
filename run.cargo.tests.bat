@echo off
setlocal

rem Path to the Rust workspace (contains Cargo.toml and .cargo/config.toml).
set "PROJECT_DIR=%~dp0src-tauri"
rem File with LLVM override for the dev profile (avoids the flaky cranelift backend).
set "CFG=%PROJECT_DIR%\.cargo\config.toml"

rem Ensure a nightly toolchain is used (required by cargo-features in Cargo.toml).
set "RUSTUP_TOOLCHAIN=nightly-x86_64-pc-windows-msvc"

if not exist "%CFG%" (
    mkdir "%PROJECT_DIR%\.cargo" 2>nul
    (
        echo [profile.dev]
        echo codegen-backend = "llvm"
    ) > "%CFG%"
    set "CFG_CREATED=1"
)

pushd "%PROJECT_DIR%"
cargo test --lib %*
set "EXIT_CODE=%ERRORLEVEL%"
popd

if "%CFG_CREATED%"=="1" (
    del /q "%CFG%" 2>nul
    rmdir "%PROJECT_DIR%\.cargo" 2>nul
)

exit /b %EXIT_CODE%
