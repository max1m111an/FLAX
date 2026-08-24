@echo off
setlocal EnableExtensions

rem ============================================================
rem Скрипт для запуска Tauri проекта FLAX
rem Использование: run.bat [--upd] [--help]
rem Файл должен быть сохранён в кодировке CP866 (OEM)
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
echo [ERROR] Неизвестный аргумент: %~1
echo Используйте --help для справки
exit /b 1

:args_done
if "%SHOW_HELP%"=="true" goto :show_help

echo =========================================
echo   Запуск проекта FLAX
echo =========================================

rem --- Проверяем наличие необходимых инструментов ---
where npm >nul 2>nul || goto :no_npm
where rustup >nul 2>nul || goto :no_rustup

rem --- Проверяем, что мы в корне проекта ---
if not exist "package.json" (
    echo [ERROR] package.json не найден! Запустите скрипт из корня проекта.
    exit /b 1
)
if not exist "src-tauri" (
    echo [ERROR] Папка src-tauri не найдена! Запустите скрипт из корня проекта.
    exit /b 1
)

if "%UPD%"=="false" (
    echo.
    echo [WARN] Быстрый запуск - обновления пропущены
    echo [INFO] Используйте --upd для обновления зависимостей
    goto :run_project
)

echo [INFO] Режим запуска с обновлением зависимостей

rem --- Обновляем Rust ---
echo.
echo [INFO] Обновление Rust ^(rustup update^)...
rustup update
if errorlevel 1 goto :upd_failed
echo [OK] Rust успешно обновлен

rem --- Устанавливаем зависимости npm ---
echo.
echo [INFO] Установка npm зависимостей...
call npm install
if errorlevel 1 goto :npm_failed
echo [OK] npm зависимости установлены

:run_project
rem --- Запускаем проект ---
echo.
echo [INFO] Запуск Tauri в режиме разработки...
echo =========================================
call npm run tauri dev
if errorlevel 1 goto :dev_failed
echo.
echo [OK] Проект успешно запущен!
exit /b 0

:show_help
echo Использование: run.bat [ОПЦИИ]
echo.
echo Опции:
echo   --upd       Обновить Rust и выполнить npm install перед запуском
echo   --help, -h  Показать эту справку
echo.
echo Примеры:
echo   run.bat           Быстрый запуск без обновлений
echo   run.bat --upd     Запуск с обновлением зависимостей
exit /b 0

:no_npm
echo [ERROR] npm не найден! Установите Node.js
exit /b 1

:no_rustup
echo [ERROR] rustup не найден! Установите Rust
exit /b 1

:upd_failed
echo [ERROR] Ошибка при обновлении Rust
exit /b 1

:npm_failed
echo [ERROR] Ошибка при установке npm зависимостей
exit /b 1

:dev_failed
echo.
echo [ERROR] Ошибка при запуске проекта
exit /b 1
