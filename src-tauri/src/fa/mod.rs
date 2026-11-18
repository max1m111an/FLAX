//! Слой конечных автоматов: реализация `FA`, её импорт/экспорт в JFLAP XML
//! (`.jff`) и соответствующие Tauri-команды.

pub mod api;
pub mod jff;
pub mod types;

#[cfg(test)]
mod tests;
