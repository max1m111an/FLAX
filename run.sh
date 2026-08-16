#!/bin/bash

# Скрипт для запуска Tauri проекта FLAX
# Использование: ./run.sh [--upd] [--help]

set -e  # Останавливаем выполнение при любой ошибке

# Цвета для вывода
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Функции для вывода
print_success() { echo -e "${GREEN}✅ $1${NC}"; }
print_error() { echo -e "${RED}❌ $1${NC}"; }
print_info() { echo -e "${BLUE}ℹ️ $1${NC}"; }
print_warning() { echo -e "${YELLOW}⚠️ $1${NC}"; }

# Парсим аргументы
UPD=false
SHOW_HELP=false

for arg in "$@"; do
    case $arg in
        --upd)
            UPD=true
            ;;
        --help|-h)
            SHOW_HELP=true
            ;;
        *)
            echo -e "${RED}❌ Неизвестный аргумент: $arg${NC}"
            echo "Используйте --help для справки"
            exit 1
            ;;
    esac
done

if [ "$SHOW_HELP" = true ]; then
    echo "Использование: ./run.sh [ОПЦИИ]"
    echo ""
    echo "Опции:"
    echo "  --upd       Обновить Rust и npm install перед запуском"
    echo "  --help, -h  Показать эту справку"
    echo ""
    echo "Примеры:"
    echo "  ./run.sh           # Быстрый запуск без обновлений"
    echo "  ./run.sh --upd     # Запуск с обновлением зависимостей"
    exit 0
fi

echo "========================================="
echo "🚀 Запуск проекта FLAX"
echo "========================================="

# Проверяем наличие необходимых инструментов
if ! command -v npm &> /dev/null; then
    print_error "npm не найден! Установите Node.js"
    exit 1
fi

if ! command -v rustup &> /dev/null; then
    print_error "rustup не найден! Установите Rust"
    exit 1
fi

# Проверяем, что мы в корне проекта
if [ ! -f "package.json" ]; then
    print_error "package.json не найден! Запустите скрипт из корня проекта."
    exit 1
fi

if [ ! -d "src-tauri" ]; then
    print_error "Папка src-tauri не найдена! Запустите скрипт из корня проекта."
    exit 1
fi

if [ "$UPD" = true ]; then
    print_info "Режим запуска с обновлением зависимостей"

    # Обновляем Rust
    echo ""
    print_info "Обновление Rust (rustup update)..."
    if rustup update; then
        print_success "Rust успешно обновлен"
    else
        print_error "Ошибка при обновлении Rust"
        exit 1
    fi

    # Устанавливаем зависимости npm
    echo ""
    print_info "Установка npm зависимостей..."
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

# Запускаем проект
echo ""
print_info "Запуск Tauri в режиме разработки..."
echo "========================================="

if npm run tauri dev; then
    echo ""
else
    echo ""
    print_error "❌ Ошибка при запуске проекта"
    exit 1
fi
