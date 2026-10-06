//! JFLAP XML (`.jff`) для конечных автоматов: экспорт (`to_jff`) и импорт
//! (`parse_jff`).
//!
//! Поддерживается только `<type>fa</type>`; пустой или отсутствующий
//! `<read>` декодируется как эпсилон-переход (`$`). Каркас документа,
//! состояния и концы переходов берутся из `crate::core::jff`.

pub mod types;

#[cfg(test)]
mod tests;

use std::collections::HashSet;

use crate::core::api::generate_id;
use crate::core::jff::{
    document_footer, document_header, find_automaton, parse_states, parse_transition_endpoints,
    read_type, transition_read, write_states, write_transition, write_transitions_header,
};
use crate::core::types::{AutomatonData, StateData, TransitionData};
use crate::fa::types::EPSILON;

pub use types::JffParsed;

pub fn to_jff(data: &AutomatonData) -> String {
    let mut s = document_header("fa");
    write_states(&mut s, &data.states);
    write_transitions_header(&mut s);
    for t in &data.transitions {
        let read = if t.symbol == EPSILON {
            None
        } else {
            Some(t.symbol.to_string())
        };
        write_transition(&mut s, t.from, t.to, read.as_deref());
    }
    s.push_str(document_footer());
    s
}

pub fn parse_jff(xml: &str) -> Result<JffParsed, String> {
    let doc = roxmltree::Document::parse(xml).map_err(|e| format!("Некорректный XML: {}", e))?;
    let root = doc.root_element();
    if root.tag_name().name() != "structure" {
        return Err("Ожидался корневой элемент <structure>".to_string());
    }

    let kind = read_type(root);
    if kind != "fa" {
        return Err(format!(
            "Поддерживается только конечный автомат (type=\"fa\"), найден type=\"{}\"",
            kind
        ));
    }

    let automaton = find_automaton(root)?;
    let states = parse_states(automaton)?;

    let mut used: HashSet<i32> = HashSet::new();
    let mut transitions: Vec<TransitionData> = Vec::new();
    let mut alphabet: Vec<char> = Vec::new();

    for child in automaton.children().filter(|n| n.is_element()) {
        if child.tag_name().name() != "transition" {
            continue;
        }

        let (from, to) = parse_transition_endpoints(child)?;
        let read = transition_read(child);

        let symbol = if read.is_empty() {
            EPSILON
        } else {
            if read.chars().count() != 1 {
                return Err(format!(
                    "Символ перехода '{}' должен быть одним символом",
                    read
                ));
            }
            read.chars().next().unwrap()
        };

        if symbol != EPSILON && !alphabet.contains(&symbol) {
            alphabet.push(symbol);
        }

        let id = generate_id(&used);
        used.insert(id);
        transitions.push(TransitionData {
            id,
            from,
            to,
            symbol,
            output: None,
        });
    }

    Ok(JffParsed {
        kind,
        states,
        transitions,
        alphabet,
    })
}

/// True if the automaton is deterministic: exactly one initial state, no
/// eps-transitions and at most one transition per `(from, symbol)`.
#[allow(dead_code)]
pub fn is_deterministic(states: &[StateData], transitions: &[TransitionData]) -> bool {
    if states.iter().filter(|s| s.isInitial).count() != 1 {
        return false;
    }
    let mut seen: HashSet<(i32, char)> = HashSet::new();
    for t in transitions {
        if t.symbol == EPSILON {
            return false;
        }
        if !seen.insert((t.from, t.symbol)) {
            return false;
        }
    }
    true
}
