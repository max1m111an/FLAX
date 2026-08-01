use crate::structs::data_models::AutomatonData;
use crate::structs::nfa::EPSILON;

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn fmt_coord(v: f32) -> String {
    let s = format!("{}", v);
    if s.contains('.') {
        s
    } else {
        format!("{}.0", s)
    }
}

pub fn to_jff(data: &AutomatonData) -> String {
    let mut s = String::new();
    s.push_str(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><!--Created with JFLAP 7.1.--><structure>\n",
    );
    s.push_str("\t<type>fa</type>\n");
    s.push_str("\t<automaton>\n");
    s.push_str("\t\t<!--The list of states.-->\n");
    for st in &data.states {
        s.push_str(&format!(
            "\t\t<state id=\"{}\" name=\"{}\">\n",
            st.id,
            escape(&st.label)
        ));
        s.push_str(&format!("\t\t\t<x>{}</x>\n", fmt_coord(st.x)));
        s.push_str(&format!("\t\t\t<y>{}</y>\n", fmt_coord(st.y)));
        if st.is_initial {
            s.push_str("\t\t\t<initial/>\n");
        }
        if st.is_final {
            s.push_str("\t\t\t<final/>\n");
        }
        s.push_str("\t\t</state>\n");
    }
    s.push_str("\t\t<!--The list of transitions.-->\n");
    for t in &data.transitions {
        s.push_str("\t\t<transition>\n");
        s.push_str(&format!("\t\t\t<from>{}</from>\n", t.from));
        s.push_str(&format!("\t\t\t<to>{}</to>\n", t.to));
        if t.symbol == EPSILON.to_string() {
            s.push_str("\t\t\t<read></read>\n");
        } else {
            s.push_str(&format!("\t\t\t<read>{}</read>\n", escape(&t.symbol)));
        }
        s.push_str("\t\t</transition>\n");
    }
    s.push_str("\t</automaton>\n");
    s.push_str("</structure>\n");
    s
}
