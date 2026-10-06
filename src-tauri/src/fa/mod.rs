//! Finite automaton layer: the `FA` implementation, its JFLAP XML (`.jff`)
//! import/export and the corresponding Tauri commands.

pub mod api;
pub mod jff;
pub mod types;

#[cfg(test)]
mod tests;
