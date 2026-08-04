use std::collections::HashSet;

use roxmltree::Node;

use crate::id_gen;
use crate::structs::data_models::{AutomatonData, AutomatonKind, StateData, TransitionData};
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

#[derive(Debug)]
pub struct JffParsed {
    #[allow(dead_code)]
    pub kind: String,
    pub states: Vec<StateData>,
    pub transitions: Vec<TransitionData>,
    pub alphabet: Vec<char>,
}

fn element_text(node: Node, tag: &str) -> Option<String> {
    for child in node.children().filter(|n| n.is_element()) {
        if child.tag_name().name() == tag {
            return Some(child.text().unwrap_or("").to_string());
        }
    }
    None
}

fn parse_attr_i32(node: Node, attr: &str) -> Result<i32, String> {
    node.attribute(attr)
        .ok_or_else(|| format!("У <state> отсутствует атрибут '{}'", attr))?
        .parse::<i32>()
        .map_err(|_| format!("Невалидный атрибут '{}'", attr))
}

pub fn parse_jff(xml: &str) -> Result<JffParsed, String> {
    let doc = roxmltree::Document::parse(xml)
        .map_err(|e| format!("Некорректный XML: {}", e))?;
    let root = doc.root_element();
    if root.tag_name().name() != "structure" {
        return Err("Ожидался корневой элемент <structure>".to_string());
    }

    let kind = element_text(root, "type").unwrap_or_default();
    if kind != "fa" {
        return Err(format!(
            "Поддерживается только конечный автомат (type=\"fa\"), найден type=\"{}\"",
            kind
        ));
    }

    let automaton = root
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "automaton")
        .ok_or("Не найден элемент <automaton>")?;

    let mut states: Vec<StateData> = Vec::new();
    let mut transitions: Vec<(i32, i32, String)> = Vec::new();

    for child in automaton.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "state" => {
                let id = parse_attr_i32(child, "id")?;
                let label = child
                    .attribute("name")
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| format!("q{}", id));

                let mut x = 0.0f32;
                let mut y = 0.0f32;
                let mut is_initial = false;
                let mut is_final = false;

                for s in child.children().filter(|n| n.is_element()) {
                    match s.tag_name().name() {
                        "x" => {
                            if let Some(t) = s.text() {
                                x = t.trim().parse().unwrap_or(0.0);
                            }
                        }
                        "y" => {
                            if let Some(t) = s.text() {
                                y = t.trim().parse().unwrap_or(0.0);
                            }
                        }
                        "initial" => is_initial = true,
                        "final" => is_final = true,
                        _ => {}
                    }
                }

                states.push(StateData {
                    id,
                    label,
                    x,
                    y,
                    is_initial,
                    is_final,
                });
            }
            "transition" => {
                let from = element_text(child, "from")
                    .and_then(|t| t.trim().parse::<i32>().ok())
                    .ok_or("У <transition> отсутствует валидный <from>")?;
                let to = element_text(child, "to")
                    .and_then(|t| t.trim().parse::<i32>().ok())
                    .ok_or("У <transition> отсутствует валидный <to>")?;
                let read = element_text(child, "read")
                    .map(|t| t.trim().to_string())
                    .unwrap_or_default();
                transitions.push((from, to, read));
            }
            _ => {}
        }
    }

    let mut used: HashSet<i32> = HashSet::new();
    let mut parsed_transitions: Vec<TransitionData> = Vec::new();
    let mut alphabet: Vec<char> = Vec::new();

    for (from, to, read) in transitions {
        let symbol = if read.is_empty() {
            EPSILON.to_string()
        } else {
            read
        };

        if symbol != EPSILON.to_string() {
            let chars: Vec<char> = symbol.chars().collect();
            if chars.len() != 1 {
                return Err(format!(
                    "Символ перехода '{}' должен быть одним символом",
                    symbol
                ));
            }
            let c = chars[0];
            if !alphabet.contains(&c) {
                alphabet.push(c);
            }
        }

        let id = id_gen::generate_id(&used);
        used.insert(id);
        parsed_transitions.push(TransitionData {
            id,
            from,
            to,
            symbol,
        });
    }

    Ok(JffParsed {
        kind,
        states,
        transitions: parsed_transitions,
        alphabet,
    })
}

pub fn is_deterministic(states: &[StateData], transitions: &[TransitionData]) -> bool {
    if states.iter().filter(|s| s.is_initial).count() != 1 {
        return false;
    }
    let mut seen: HashSet<(i32, String)> = HashSet::new();
    for t in transitions {
        if t.symbol == EPSILON.to_string() {
            return false;
        }
        if !seen.insert((t.from, t.symbol.clone())) {
            return false;
        }
    }
    true
}

pub fn infer_kind(states: &[StateData], transitions: &[TransitionData]) -> AutomatonKind {
    if is_deterministic(states, transitions) {
        AutomatonKind::DFA
    } else {
        AutomatonKind::NFA
    }
}
