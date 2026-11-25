//! JFLAP XML (`.jff`) для конечных автоматов: экспорт (`to_jff`) и разбор
//! `<automaton>` (`parse_automaton`).
//!
//! Непустой `<read>` целиком становится меткой перехода (как в JFLAP), пустой
//! или отсутствующий декодируется как эпсилон-переход (`$`).
//! Выбор ветки по `<type>` (автомат или другой формальный язык) делает
//! диспетчер `crate::core::jff::parse_jff`. Каркас документа, состояния и
//! концы переходов берутся из `crate::core::jff`.

#[cfg(test)]
mod tests;

use roxmltree::Node;

use crate::core::jff::{
    JffAutomaton, document_footer, document_header, find_automaton, parse_states,
    parse_transitions, write_states, write_transition, write_transitions_header,
};
use crate::core::types::{AutomatonData, StateData, TransitionData, check_label};
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

    // Пустой или отсутствующий <read> — eps-переход; остальной текст целиком
    // становится меткой, которую переход прочитает за один шаг.
    let transitions = parse_transitions(automaton, |_node, read| {
        let symbol = if read.is_empty() {
            EPSILON.to_string()
        } else {
            check_label(read, "Метка перехода")?
        };
        Ok((symbol, None))
    })?;

    Ok(JffAutomaton {
        states,
        transitions,
    })
}

/// `true`, если автомат детерминирован: ровно одно начальное состояние, нет
/// eps-переходов, повторов метки из одного состояния и пар меток, где одна
/// является префиксом другой.
#[allow(dead_code)]
pub fn is_deterministic(states: &[StateData], transitions: &[TransitionData]) -> bool {
    if states.iter().filter(|s| s.isInitial).count() != 1 {
        return false;
    }
    for t in transitions {
        if t.symbol == EPSILON {
            return false;
        }
        let conflicts = transitions.iter().any(|other| {
            other.id != t.id
                && other.from == t.from
                && (other.symbol == t.symbol
                    || other.symbol.starts_with(&t.symbol)
                    || t.symbol.starts_with(&other.symbol))
        });
        if conflicts {
            return false;
        }
    }
    true
}
