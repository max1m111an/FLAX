//! JFLAP XML (`.jff`) для машин Мили: экспорт (`to_jff`) и разбор
//! `<automaton>` (`parse_automaton`).
//!
//! `<type>mealy</type>`, переходы пишут входной символ в `<read>`, а выходной
//! — в `<transout>`. У машины Мили нет допускающих состояний, поэтому
//! `<final/>` не сериализуется. Выбор ветки по `<type>` делает диспетчер
//! `crate::core::jff::parse_jff`; каркас документа и состояния берутся из
//! `crate::core::jff`.

#[cfg(test)]
mod tests;

use roxmltree::Node;

use crate::core::jff::{
    JffAutomaton, document_footer, document_header, find_automaton, one_char, parse_states,
    parse_transitions, transition_output, write_states, write_transition_with_output,
    write_transitions_header,
};
use crate::core::types::{AutomatonData, StateData};

pub fn to_jff(data: &AutomatonData) -> String {
    let mut s = document_header("mealy");

    let states: Vec<StateData> = data
        .states
        .iter()
        .cloned()
        .map(|mut st| {
            st.isFinal = Some(false);
            st
        })
        .collect();
    write_states(&mut s, &states);

    write_transitions_header(&mut s);
    for t in &data.transitions {
        let input = t.symbol.to_string();
        let output = t.output.map(|c| c.to_string()).unwrap_or_default();
        write_transition_with_output(&mut s, t.from, t.to, &input, &output);
    }

    s.push_str(document_footer());
    s
}

/// Разбирает `<automaton>` машины Мили: входной символ из `<read>`, выходной
/// из `<transout>`. Вызывается диспетчером [`crate::core::jff::parse_jff`].
pub(crate) fn parse_automaton(root: Node) -> Result<JffAutomaton, String> {
    let automaton = find_automaton(root)?;
    let states = parse_states(automaton)?;

    let transitions = parse_transitions(automaton, |node, read| {
        let input = one_char(read, "Входной символ перехода")?;
        let output = one_char(&transition_output(node), "Выходной символ перехода")?;
        Ok((input, Some(output)))
    })?;

    Ok(JffAutomaton {
        states,
        transitions,
    })
}
