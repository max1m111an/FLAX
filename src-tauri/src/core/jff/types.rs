//! Data produced by the JFLAP XML (`.jff`) parser.

use crate::core::types::{AutomatonType, StateData, TransitionData};

/// Parsed contents of an automaton `<automaton>` block.
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct JffAutomaton {
    pub states: Vec<StateData>,
    pub transitions: Vec<TransitionData>,
}

/// Placeholder for non-automaton formal languages (grammar, Turing machine,
/// PDA, ...): they are recognised as "not an automaton" but not parsed yet.
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct JffGrammar {
    // ...
}

/// Result of parsing a JFLAP `<structure>`: either a real automaton (with its
/// [`AutomatonType`]) or a mock for every other formal-language `<type>`.
#[derive(Debug)]
#[allow(non_snake_case)]
pub enum JffParsed {
    Automaton(AutomatonType, JffAutomaton),
    Grammar(JffGrammar),
}
