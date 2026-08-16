#!/bin/bash

# Скрипт для запуска Tauri проекта

echo "🚀 Запуск проекта FLAX"
echo "================================"

echo "📦 Обновление Rust (rustup update)..."
rustup update

echo "📦 Установка npm зависимостей..."
npm install

echo "🚀 Запуск Tauri в режиме разработки..."
npm run tauri dev