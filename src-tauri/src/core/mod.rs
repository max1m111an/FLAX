//! Ядро доменного слоя: общие типы, реестр автоматов, вспомогательные функции
//! и общая инфраструктура JFLAP XML, используемая слоями `fa` и `mealy`.

pub mod api;
pub mod jff;
pub mod types;

#[cfg(test)]
mod tests;
