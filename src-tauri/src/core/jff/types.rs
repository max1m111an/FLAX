//! Данные, получаемые парсером JFLAP XML (`.jff`).

use crate::core::types::{AutomatonType, StateData, TransitionData};

/// Разобранное содержимое блока `<automaton>`.
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct JffAutomaton {
    pub states: Vec<StateData>,
    pub transitions: Vec<TransitionData>,
}

/// Заглушка для формальных языков, не являющихся автоматами (грамматика, машина
/// Тьюринга, МП-автомат, ...): они распознаются как «не автомат», но пока не
/// разбираются.
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct JffGrammar {
    // ...
}

/// Результат разбора JFLAP `<structure>`: либо настоящий автомат (с его
/// [`AutomatonType`]), либо заглушка для любого другого `<type>` формального языка.
#[derive(Debug)]
#[allow(non_snake_case)]
pub enum JffParsed {
    Automaton(AutomatonType, JffAutomaton),
    Grammar(JffGrammar),
}
