#!/bin/bash

# Скрипт для запуска Tauri проекта с nightly и Cranelift
# Использование: ./run.dev.sh

set -e  # Останавливаем выполнение при любой ошибке

echo "🚀 Запуск Tauri FLAX с nightly + Cranelift"
echo "============================================"

echo "📦 Обновление nightly Rust..."
rustup update nightly

echo "🔧 Установка rustc-codegen-cranelift-preview для nightly..."
rustup component add rustc-codegen-cranelift-preview --toolchain nightly

echo "🧹 Очистка src-tauri..."
cd src-tauri
cargo +nightly clean
cd ..

echo "🌙 Установка nightly как тулчейн по умолчанию..."
rustup default nightly

echo "📦 Установка npm зависимостей..."
npm install

echo "🚀 Запуск Tauri dev с nightly + Cranelift..."
npm run tauri dev
