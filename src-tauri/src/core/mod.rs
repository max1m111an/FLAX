//! Core domain layer: shared types, the automaton registry, small helpers
//! and the shared JFLAP XML plumbing used by the `fa` and `mealy` layers.

pub mod api;
pub mod jff;
pub mod types;

#[cfg(test)]
mod tests;
