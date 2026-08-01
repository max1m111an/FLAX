use std::env;
use std::fs;
use std::path::Path;

use crate::jff::to_jff;
use crate::structs::data_models::{AutomatonData, AutomatonKind, StateData, TransitionData};

fn even_a_data() -> AutomatonData {
    AutomatonData {
        id: 1,
        name: "Even A".to_string(),
        kind: AutomatonKind::DFA,
        states: vec![
            StateData {
                id: 0,
                label: "q0".to_string(),
                x: 154.0,
                y: 136.0,
                is_initial: true,
                is_final: true,
            },
            StateData {
                id: 1,
                label: "q1".to_string(),
                x: 308.0,
                y: 147.0,
                is_initial: false,
                is_final: false,
            },
        ],
        transitions: vec![
            TransitionData { id: 1, from: 0, to: 1, symbol: "a".to_string(), label: None },
            TransitionData { id: 2, from: 1, to: 0, symbol: "a".to_string(), label: None },
            TransitionData { id: 3, from: 0, to: 0, symbol: "b".to_string(), label: None },
            TransitionData { id: 4, from: 1, to: 1, symbol: "b".to_string(), label: None },
        ],
        alphabet: vec!['a', 'b'],
    }
}

#[test]
fn starts_with_xml_declaration() {
    let xml = to_jff(&even_a_data());
    assert!(xml.starts_with(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><!--Created with JFLAP 7.1.--><structure>"
    ));
}

#[test]
fn uses_fa_type_for_both_kinds() {
    let mut data = even_a_data();
    data.kind = AutomatonKind::NFA;
    let xml = to_jff(&data);
    assert!(xml.contains("\t<type>fa</type>\n"));
}

#[test]
fn serializes_states_with_attributes() {
    let xml = to_jff(&even_a_data());

    assert!(xml.contains("<state id=\"0\" name=\"q0\">"));
    assert!(xml.contains("<state id=\"1\" name=\"q1\">"));
    assert!(xml.contains("\t\t\t<x>154.0</x>"));
    assert!(xml.contains("\t\t\t<y>136.0</y>"));
    assert!(xml.contains("\t\t\t<x>308.0</x>"));
    assert!(xml.contains("\t\t\t<y>147.0</y>"));
}

#[test]
fn serializes_initial_and_final_flags() {
    let xml = to_jff(&even_a_data());

    let q0_block: Vec<&str> = xml.split("</state>").collect();
    assert!(q0_block[0].contains("<initial/>"));
    assert!(q0_block[0].contains("<final/>"));

    let q1_block: Vec<&str> = xml.split("</state>").collect();
    assert!(!q1_block[1].contains("<initial/>"));
    assert!(!q1_block[1].contains("<final/>"));
}

#[test]
fn serializes_transitions() {
    let xml = to_jff(&even_a_data());

    assert_eq!(xml.matches("<transition>").count(), 4);
    assert!(xml.contains("\t\t\t<from>0</from>\n\t\t\t<to>1</to>\n\t\t\t<read>a</read>"));
    assert!(xml.contains("\t\t\t<from>1</from>\n\t\t\t<to>1</to>\n\t\t\t<read>b</read>"));
}

#[test]
fn epsilon_transition_written_as_empty_read() {
    let mut data = even_a_data();
    data.transitions.push(TransitionData {
        id: 5,
        from: 0,
        to: 1,
        symbol: "$".to_string(),
        label: None,
    });

    let xml = to_jff(&data);
    assert!(xml.contains("\t\t\t<read></read>"));
    assert!(!xml.contains("<read>$</read>"));
}

#[test]
fn escapes_special_characters() {
    let mut data = even_a_data();
    data.states[0].label = "q&<0>\"'".to_string();
    data.transitions[0].symbol = "<&>".to_string();

    let xml = to_jff(&data);
    assert!(xml.contains("name=\"q&amp;&lt;0&gt;&quot;&apos;\""));
    assert!(xml.contains("<read>&lt;&amp;&gt;</read>"));
}

#[test]
fn coordinates_with_fraction_keep_decimals() {
    let mut data = even_a_data();
    data.states[0].x = 136.5;
    data.states[1].y = -20.25;

    let xml = to_jff(&data);
    assert!(xml.contains("\t\t\t<x>136.5</x>"));
    assert!(xml.contains("\t\t\t<y>-20.25</y>"));
}

#[test]
fn no_states_and_transitions_yields_empty_automaton() {
    let data = AutomatonData {
        id: 2,
        name: "Empty".to_string(),
        kind: AutomatonKind::NFA,
        states: Vec::new(),
        transitions: Vec::new(),
        alphabet: Vec::new(),
    };

    let xml = to_jff(&data);
    assert!(xml.contains("<automaton>"));
    assert!(xml.contains("</automaton>"));
    assert!(!xml.contains("<state"));
    assert!(!xml.contains("<transition>"));
    assert!(xml.contains("</structure>"));
}

#[test]
fn writes_example_files_to_target() {
    let dfa = to_jff(&even_a_data());

    let mut nfa_data = even_a_data();
    nfa_data.kind = AutomatonKind::NFA;
    nfa_data.transitions.push(TransitionData {
        id: 5,
        from: 0,
        to: 1,
        symbol: "$".to_string(),
        label: None,
    });
    let nfa = to_jff(&nfa_data);

    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("jff_examples");
    fs::create_dir_all(&out_dir).unwrap();
    fs::write(out_dir.join("even_a_dfa.jff"), dfa).unwrap();
    fs::write(out_dir.join("nfa_with_epsilon.jff"), nfa).unwrap();
}
