#!/bin/bash

# Скрипт для запуска Tauri проекта с nightly и Cranelift
# Использование: ./run.dev.sh [--upd] [--clean] [--help]

set -e  # Останавливаем выполнение при любой ошибке

# Цвета для вывода
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Функции для вывода
print_success() { echo -e "${GREEN}✅ $1${NC}"; }
print_error() { echo -e "${RED}❌ $1${NC}"; }
print_warning() { echo -e "${YELLOW}⚠️ $1${NC}"; }
print_info() { echo -e "${BLUE}ℹ️ $1${NC}"; }
print_header() { echo -e "${CYAN}=== $1 ===${NC}"; }

# Парсим аргументы
UPD=false
CLEAN=false
SHOW_HELP=false

for arg in "$@"; do
    case $arg in
        --upd)
            UPD=true
            ;;
        --clean)
            CLEAN=true
            ;;
        --help|-h)
            SHOW_HELP=true
            ;;
        *)
            print_error "Неизвестный аргумент: $arg"
            echo "Используйте --help для справки"
            exit 1
            ;;
    esac
done

if [ "$SHOW_HELP" = true ]; then
    echo "Использование: ./run.dev.sh [ОПЦИИ]"
    echo ""
    echo "Опции:"
    echo "  --upd       Обновить nightly Rust и npm install перед запуском"
    echo "  --clean     Очистить проект (cargo clean) перед сборкой"
    echo "  --help, -h  Показать эту справку"
    echo ""
    echo "Примеры:"
    echo "  ./run.dev.sh               # Быстрый запуск с nightly"
    echo "  ./run.dev.sh --upd         # Запуск с обновлением зависимостей"
    echo "  ./run.dev.sh --clean       # Запуск с очисткой проекта"
    echo "  ./run.dev.sh --upd --clean # Полный запуск с обновлениями и очисткой"
    exit 0
fi

echo "========================================="
echo "🚀 Запуск Tauri FLAX с nightly + Cranelift"
echo "========================================="

# Проверяем наличие необходимых инструментов
if ! command -v rustup &> /dev/null; then
    print_error "rustup не найден! Установите Rust: https://rustup.rs/"
    exit 1
fi

if ! command -v npm &> /dev/null; then
    print_error "npm не найден! Установите Node.js: https://nodejs.org/"
    exit 1
fi

# Проверяем, что мы в корне проекта
if [ ! -d "src-tauri" ]; then
    print_error "Папка src-tauri не найдена! Запустите скрипт из корня проекта."
    exit 1
fi

if [ ! -f "package.json" ]; then
    print_error "package.json не найден! Запустите скрипт из корня проекта."
    exit 1
fi

# Сохраняем текущий тулчейн для восстановления
ORIGINAL_TOOLCHAIN=$(rustup default | awk '{print $1}')
print_info "Текущий тулчейн: $ORIGINAL_TOOLCHAIN"

# Функция для восстановления тулчейна
cleanup() {
    if [ -n "$ORIGINAL_TOOLCHAIN" ]; then
        print_info "Восстанавливаем исходный тулчейн: $ORIGINAL_TOOLCHAIN"
        rustup default $ORIGINAL_TOOLCHAIN 2>/dev/null || true
    fi
}

# Устанавливаем trap для восстановления при завершении
trap cleanup EXIT

if [ "$UPD" = true ]; then
    print_info "Режим запуска с обновлением зависимостей"

    # 1. Обновляем nightly
    echo ""
    print_header "Обновление nightly Rust"
    if rustup update nightly; then
        print_success "Nightly успешно обновлен"
    else
        print_error "Ошибка при обновлении nightly"
        exit 1
    fi

    # 2. Добавляем компонент Cranelift для nightly
    echo ""
    print_header "Установка компонента Cranelift"
    if rustup component add rustc-codegen-cranelift-preview --toolchain nightly; then
        print_success "Cranelift компонент успешно установлен"
    else
        print_warning "Не удалось установить Cranelift компонент"
        print_warning "Проверьте: https://github.com/rust-lang/rustc_codegen_cranelift"
        print_warning "Продолжаем без Cranelift..."
    fi

    # 3. Устанавливаем зависимости npm
    echo ""
    print_header "Установка npm зависимостей"
    if npm install; then
        print_success "npm зависимости установлены"
    else
        print_error "Ошибка при установке npm зависимостей"
        exit 1
    fi
else
    echo ""
    print_warning "Быстрый запуск (обновления пропущены)"
    print_info "Используйте --upd для обновления зависимостей"
fi

# 4. Очищаем проект в src-tauri (если указан --clean)
if [ "$CLEAN" = true ]; then
    echo ""
    print_header "Очистка проекта в src-tauri"
    if cd src-tauri; then
        cargo +nightly clean || print_warning "Очистка не удалась (продолжаем...)"
        cd ..
        print_success "Очистка завершена"
    else
        print_error "Не удалось перейти в папку src-tauri"
        exit 1
    fi
else
    echo ""
    print_warning "Пропускаем очистку проекта (используйте --clean для очистки)"
fi

# 5. Устанавливаем nightly по умолчанию
echo ""
print_header "Установка nightly как тулчейн по умолчанию"
if rustup default nightly; then
    print_success "Nightly установлен как тулчейн по умолчанию"
else
    print_error "Ошибка при установке nightly по умолчанию"
    exit 1
fi

# 6. Запускаем Tauri dev
echo ""
print_header "Запуск Tauri dev с nightly + Cranelift"
echo "========================================="

# Проверяем, установлен ли Cranelift
if rustup component list --toolchain nightly 2>/dev/null | grep -q "rustc-codegen-cranelift-preview.*installed"; then
    print_info "Используем Cranelift для ускорения сборки"
    export RUSTFLAGS="-Z codegen-backend=cranelift"
else
    print_warning "Cranelift не установлен, используем стандартный бекенд"
fi

# Запускаем Tauri
if npm run tauri dev; then
    echo ""
else
    echo ""
    print_error "❌ Ошибка при запуске Tauri"
    exit 1
fi
