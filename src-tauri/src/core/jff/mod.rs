//! Общая логика JFLAP XML (`.jff`): XML-утилиты, каркас документа,
//! разбор и запись состояний и концов переходов.
//!
//! Здесь живёт только то, что одинаково для всех типов автоматов.
//! Специфика формата конкретного типа (допустимый `<type>`, символы
//! переходов, эпсилон, алфавиты) — в модуле этого типа, например
//! `crate::fa::jff`.

use roxmltree::Node;

use crate::core::types::StateData;

pub fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub fn fmt_coord(v: f32) -> String {
    let s = format!("{}", v);
    if s.contains('.') {
        s
    } else {
        format!("{}.0", s)
    }
}

/// Текст первого прямого потомка с тегом `tag`, если такой есть.
pub fn element_text(node: Node, tag: &str) -> Option<String> {
    for child in node.children().filter(|n| n.is_element()) {
        if child.tag_name().name() == tag {
            return Some(child.text().unwrap_or("").to_string());
        }
    }
    None
}

pub fn parse_attr_i32(node: Node, attr: &str) -> Result<i32, String> {
    node.attribute(attr)
        .ok_or_else(|| format!("У <state> отсутствует атрибут '{}'", attr))?
        .parse::<i32>()
        .map_err(|_| format!("Невалидный атрибут '{}'", attr))
}

/// Пролог документа: XML-декларация, `<structure>`, `<type>` и открытие
/// `<automaton>`.
pub fn document_header(kind: &str) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><!--Created with JFLAP 7.1.--><structure>\n\t<type>{}</type>\n\t<automaton>\n",
        kind
    )
}

/// Эпилог: закрытие `<automaton>` и `<structure>`.
pub fn document_footer() -> &'static str {
    "\t</automaton>\n</structure>\n"
}

/// Сырое содержимое `<type>` (пустая строка, если элемент отсутствует).
pub fn read_type(root: Node) -> String {
    element_text(root, "type").unwrap_or_default()
}

/// Первый прямой потомок `<automaton>`.
pub fn find_automaton<'a, 'input>(root: Node<'a, 'input>) -> Result<Node<'a, 'input>, String> {
    root.children()
        .find(|n| n.is_element() && n.tag_name().name() == "automaton")
        .ok_or_else(|| "Не найден элемент <automaton>".to_string())
}

/// JFLAP-комментарий перед списком переходов.
pub fn write_transitions_header(out: &mut String) {
    out.push_str("\t\t<!--The list of transitions.-->\n");
}

/// Список состояний с JFLAP-комментарием.
pub fn write_states(out: &mut String, states: &[StateData]) {
    out.push_str("\t\t<!--The list of states.-->\n");
    for st in states {
        out.push_str(&format!(
            "\t\t<state id=\"{}\" name=\"{}\">\n",
            st.id,
            escape(&st.label)
        ));
        out.push_str(&format!("\t\t\t<x>{}</x>\n", fmt_coord(st.x)));
        out.push_str(&format!("\t\t\t<y>{}</y>\n", fmt_coord(st.y)));
        if st.isInitial {
            out.push_str("\t\t\t<initial/>\n");
        }
        if st.isFinal {
            out.push_str("\t\t\t<final/>\n");
        }
        out.push_str("\t\t</state>\n");
    }
}

/// Один `<transition>`. `read: None` пишет пустой `<read></read>`
/// (эпсилон у FA, отсутствие входа у других типов).
pub fn write_transition(out: &mut String, from: i32, to: i32, read: Option<&str>) {
    out.push_str("\t\t<transition>\n");
    out.push_str(&format!("\t\t\t<from>{}</from>\n", from));
    out.push_str(&format!("\t\t\t<to>{}</to>\n", to));
    match read {
        Some(text) => out.push_str(&format!("\t\t\t<read>{}</read>\n", escape(text))),
        None => out.push_str("\t\t\t<read></read>\n"),
    }
    out.push_str("\t\t</transition>\n");
}

/// Все `<state>` внутри `<automaton>` в порядке документа.
pub fn parse_states(automaton: Node) -> Result<Vec<StateData>, String> {
    let mut states: Vec<StateData> = Vec::new();

    for child in automaton.children().filter(|n| n.is_element()) {
        if child.tag_name().name() != "state" {
            continue;
        }

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
            isInitial: is_initial,
            isFinal: is_final,
        });
    }

    Ok(states)
}

/// `<from>` и `<to>` одного `<transition>`.
pub fn parse_transition_endpoints(node: Node) -> Result<(i32, i32), String> {
    let from = element_text(node, "from")
        .and_then(|t| t.trim().parse::<i32>().ok())
        .ok_or("У <transition> отсутствует валидный <from>")?;
    let to = element_text(node, "to")
        .and_then(|t| t.trim().parse::<i32>().ok())
        .ok_or("У <transition> отсутствует валидный <to>")?;
    Ok((from, to))
}

/// Обрезанный текст `<read>`; отсутствие элемента — пустая строка.
pub fn transition_read(node: Node) -> String {
    element_text(node, "read")
        .map(|t| t.trim().to_string())
        .unwrap_or_default()
}
