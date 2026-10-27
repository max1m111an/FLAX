//! Data produced by the JFLAP XML (`.jff`) parser.

use crate::core::types::{StateData, TransitionData};

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct JffParsed {
    /// Whether the JFLAP `<type>` describes an automaton rather than another
    /// formal language.
    pub isAutomaton: bool,
    pub states: Vec<StateData>,
    pub transitions: Vec<TransitionData>,
}
