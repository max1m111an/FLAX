@echo off
setlocal EnableExtensions

rem ============================================================
rem Скрипт для запуска Tauri проекта с nightly и Cranelift
rem Использование: run.dev.bat [--upd] [--clean] [--help]
rem Файл должен быть сохранён в кодировке CP866 (OEM)
rem ============================================================

set "UPD=false"
set "CLEAN=false"
set "SHOW_HELP=false"
set "ORIGINAL_TOOLCHAIN="
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
echo [ERROR] Неизвестный аргумент: %~1
echo Используйте --help для справки
exit /b 1

:args_done
if "%SHOW_HELP%"=="true" goto :show_help

echo =========================================
echo   Запуск Tauri FLAX с nightly + Cranelift
echo =========================================

rem --- Проверяем наличие необходимых инструментов ---
where rustup >nul 2>nul || goto :no_rustup
where npm >nul 2>nul || goto :no_npm

rem --- Проверяем, что мы в корне проекта ---
if not exist "src-tauri" (
    echo [ERROR] Папка src-tauri не найдена! Запустите скрипт из корня проекта.
    exit /b 1
)
if not exist "package.json" (
    echo [ERROR] package.json не найден! Запустите скрипт из корня проекта.
    exit /b 1
)

rem --- Сохраняем текущий тулчейн для восстановления ---
for /f "tokens=1" %%a in ('rustup default 2^>nul') do set "ORIGINAL_TOOLCHAIN=%%a"
echo [INFO] Текущий тулчейн: %ORIGINAL_TOOLCHAIN%

if "%UPD%"=="false" (
    echo.
    echo [WARN] Быстрый запуск - обновления пропущены
    echo [INFO] Используйте --upd для обновления зависимостей
    goto :maybe_clean
)

echo [INFO] Режим запуска с обновлением зависимостей

rem --- 1. Обновляем nightly ---
echo.
echo === Обновление nightly Rust ===
rustup update nightly
if errorlevel 1 goto :err_upd_nightly
echo [OK] Nightly успешно обновлен

rem --- 2. Проверяем наличие Cranelift компонента ---
echo.
echo === Проверка компонента Cranelift ===

rem Проверяем, установлен ли уже Cranelift
rustup component list --toolchain nightly 2>nul | findstr /R /C:"rustc-codegen-cranelift-preview.*installed" >nul
if errorlevel 1 (
    echo [INFO] Cranelift компонент не установлен. Пытаемся установить...
    
    rem Пытаемся установить Cranelift
    rustup component add rustc-codegen-cranelift-preview --toolchain nightly 2>nul
    if errorlevel 1 (
        echo [WARN] Не удалось установить Cranelift компонент через rustup
        echo [WARN] Проверьте доступность: https://github.com/rust-lang/rustc_codegen_cranelift
        echo [WARN] Продолжаем без Cranelift...
        set "CRANELIFT_INSTALLED=false"
    ) else (
        echo [OK] Cranelift компонент успешно установлен
        set "CRANELIFT_INSTALLED=true"
    )
) else (
    echo [OK] Cranelift компонент уже установлен
    set "CRANELIFT_INSTALLED=true"
)

rem --- Альтернативная установка через cargo (если rustup не помог) ---
if "%CRANELIFT_INSTALLED%"=="false" (
    echo.
    echo [INFO] Пытаемся установить Cranelift через cargo...
    
    rem Проверяем наличие cargo-clif
    where cargo-clif >nul 2>nul
    if errorlevel 1 (
        echo [INFO] Устанавливаем cargo-clif...
        cargo install cargo-clif 2>nul
        if errorlevel 1 (
            echo [WARN] Не удалось установить cargo-clif
            echo [WARN] Продолжаем без Cranelift
        ) else (
            echo [OK] cargo-clif установлен
            set "CRANELIFT_INSTALLED=true"
        )
    ) else (
        echo [OK] cargo-clif уже установлен
        set "CRANELIFT_INSTALLED=true"
    )
)

rem --- 3. Устанавливаем зависимости npm ---
echo.
echo === Установка npm зависимостей ===
call npm install
if errorlevel 1 goto :err_npm_install
echo [OK] npm зависимости установлены

:maybe_clean
rem --- 4. Очищаем проект в src-tauri (если указан --clean) ---
if "%CLEAN%"=="false" (
    echo.
    echo [WARN] Пропускаем очистку проекта - используйте --clean для очистки
    goto :set_default_toolchain
)

echo.
echo === Очистка проекта в src-tauri ===
pushd src-tauri || goto :err_cd_tauri
cargo +nightly clean
if errorlevel 1 echo [WARN] Очистка не удалась - продолжаем...
popd
echo [OK] Очистка завершена

:set_default_toolchain
rem --- 5. Устанавливаем nightly по умолчанию ---
echo.
echo === Установка nightly как тулчейн по умолчанию ===
rustup default nightly
if errorlevel 1 goto :err_set_default
echo [OK] Nightly установлен как тулчейн по умолчанию

rem --- 6. Запускаем Tauri dev ---
echo.
echo === Запуск Tauri dev с nightly + Cranelift ===
echo =========================================

rem --- Проверяем, установлен ли Cranelift и настраиваем флаги ---
set "RUSTFLAGS="

if "%CRANELIFT_INSTALLED%"=="true" (
    echo [INFO] Используем Cranelift для ускорения сборки
    
    rem Проверяем, что компонент действительно установлен через rustup
    rustup component list --toolchain nightly 2>nul | findstr /R /C:"rustc-codegen-cranelift-preview.*installed" >nul
    if not errorlevel 1 (
        set "RUSTFLAGS=-Z codegen-backend=cranelift"
        echo [INFO] Cranelift активирован через RUSTFLAGS
    ) else (
        rem Проверяем наличие cargo-clif
        where cargo-clif >nul 2>nul
        if not errorlevel 1 (
            echo [INFO] Используем cargo-clif вместо cargo
            call cargo-clif tauri dev
            if errorlevel 1 goto :err_dev
            goto :success
        ) else (
            echo [WARN] Cranelift не найден, используем стандартный бекенд
        )
    )
) else (
    echo [WARN] Cranelift не установлен, используем стандартный бекенд
)

rem --- Запускаем Tauri с установленными флагами ---
if defined RUSTFLAGS (
    echo [INFO] RUSTFLAGS=%RUSTFLAGS%
    set "RUSTFLAGS=%RUSTFLAGS%"
) else (
    echo [INFO] Используем стандартную сборку без Cranelift
)

call npm run tauri dev
if errorlevel 1 goto :err_dev

:success
echo.
echo [OK] Проект успешно запущен с nightly + Cranelift!
call :restore_toolchain
exit /b 0

:show_help
echo Использование: run.dev.bat [ОПЦИИ]
echo.
echo Опции:
echo   --upd       Обновить nightly Rust, установить Cranelift и выполнить npm install
echo   --clean     Очистить проект (cargo clean) перед сборкой
echo   --help, -h  Показать эту справку
echo.
echo Примеры:
echo   run.dev.bat                Быстрый запуск с nightly
echo   run.dev.bat --upd          Запуск с обновлением зависимостей и установкой Cranelift
echo   run.dev.bat --clean        Запуск с очисткой проекта
echo   run.dev.bat --upd --clean  Полный запуск с обновлениями и очисткой
echo.
echo Примечание: Скрипт автоматически пытается установить Cranelift
echo если он не найден.
exit /b 0

:restore_toolchain
if defined ORIGINAL_TOOLCHAIN (
    echo [INFO] Восстанавливаем исходный тулчейн: %ORIGINAL_TOOLCHAIN%
    rustup default %ORIGINAL_TOOLCHAIN% >nul 2>nul
)
goto :eof

:no_rustup
echo [ERROR] rustup не найден! Установите Rust: https://rustup.rs/
exit /b 1

:no_npm
echo [ERROR] npm не найден! Установите Node.js: https://nodejs.org/
exit /b 1

:err_upd_nightly
echo [ERROR] Ошибка при обновлении nightly
call :restore_toolchain
exit /b 1

:err_npm_install
echo [ERROR] Ошибка при установке npm зависимостей
call :restore_toolchain
exit /b 1

:err_cd_tauri
echo [ERROR] Не удалось перейти в папку src-tauri
call :restore_toolchain
exit /b 1

:err_set_default
echo [ERROR] Ошибка при установке nightly по умолчанию
call :restore_toolchain
exit /b 1

:err_dev
echo.
echo [ERROR] Ошибка при запуске Tauri
call :restore_toolchain
exit /b 1