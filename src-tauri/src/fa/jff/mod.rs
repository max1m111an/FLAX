//! JFLAP XML (`.jff`) для конечных автоматов: экспорт (`to_jff`) и разбор
//! `<automaton>` (`parse_automaton`).
//!
//! Пустой или отсутствующий `<read>` декодируется как эпсилон-переход (`$`).
//! Выбор ветки по `<type>` (автомат или другой формальный язык) делает
//! диспетчер `crate::core::jff::parse_jff`. Каркас документа, состояния и
//! концы переходов берутся из `crate::core::jff`.

#[cfg(test)]
mod tests;

use std::collections::HashSet;

use roxmltree::Node;

use crate::core::jff::{
    JffAutomaton, document_footer, document_header, find_automaton, parse_states,
    parse_transitions, write_states, write_transition, write_transitions_header,
};
use crate::core::types::{AutomatonData, StateData, TransitionData, one_symbol};
use crate::fa::types::EPSILON;

pub fn to_jff(data: &AutomatonData) -> String {
    let mut s = document_header("fa");
    write_states(&mut s, &data.states);
    write_transitions_header(&mut s);
    for t in &data.transitions {
        let read = if t.symbol == EPSILON {
            None
        } else {
            Some(t.symbol.as_str())
        };
        write_transition(&mut s, t.from, t.to, read);
    }
    s.push_str(document_footer());
    s
}

/// Разбирает `<automaton>` из уже проверенного корневого элемента
/// `<structure>`. Вызывается диспетчером [`crate::core::jff::parse_jff`].
pub(crate) fn parse_automaton(root: Node) -> Result<JffAutomaton, String> {
    let automaton = find_automaton(root)?;
    let states = parse_states(automaton)?;

    // Пустой или отсутствующий <read> — эпсилон-переход.
    let transitions = parse_transitions(automaton, |_node, read| {
        let symbol = if read.is_empty() {
            EPSILON.to_string()
        } else {
            one_symbol(read, "Символ перехода")?
        };
        Ok((symbol, None))
    })?;

    Ok(JffAutomaton {
        states,
        transitions,
    })
}

/// `true`, если автомат детерминирован: ровно одно начальное состояние, нет
/// eps-переходов и не более одного перехода на каждую пару `(from, symbol)`.
#[allow(dead_code)]
pub fn is_deterministic(states: &[StateData], transitions: &[TransitionData]) -> bool {
    if states.iter().filter(|s| s.isInitial).count() != 1 {
        return false;
    }
    let mut seen: HashSet<(i32, String)> = HashSet::new();
    for t in transitions {
        if t.symbol == EPSILON {
            return false;
        }
        if !seen.insert((t.from, t.symbol.clone())) {
            return false;
        }
    }
    true
}
