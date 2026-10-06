//! Data produced by the JFLAP XML (`.jff`) parser.

use crate::core::types::{StateData, TransitionData};

#[derive(Debug)]
pub struct JffParsed {
    /// Raw JFLAP automaton type as written in `<type>` (always `fa`).
    #[allow(dead_code)]
    pub kind: String,
    pub states: Vec<StateData>,
    pub transitions: Vec<TransitionData>,
    pub alphabet: Vec<char>,
}
