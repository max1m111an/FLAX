//! JFLAP XML (`.jff`) для машин Мили: экспорт (`to_jff`).
//!
//! `<type>mealy</type>`, переходы пишут входной символ в `<read>`, а выходной
//! — в `<transout>`. У машины Мили нет допускающих состояний, поэтому
//! `<final/>` не сериализуется. Каркас документа и состояния берутся из
//! `crate::core::jff`.

#[cfg(test)]
mod tests;

use crate::core::jff::{
    document_footer, document_header, write_states, write_transition_with_output,
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
            st.isFinal = false;
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
