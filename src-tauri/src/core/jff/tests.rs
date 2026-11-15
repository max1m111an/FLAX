//! Tests for the JFLAP `<type>` dispatcher (`parse_jff`).

use super::{JffParsed, parse_jff};
use crate::core::types::AutomatonType;

fn assert_grammar_mock(xml: &str) {
    match parse_jff(xml).unwrap() {
        JffParsed::Grammar(_) => {}
        JffParsed::Automaton(_, _) => panic!("ожидалась заглушка формального языка"),
    }
}

#[test]
fn rejects_invalid_xml() {
    assert!(parse_jff("not xml at all <").is_err());
    assert!(parse_jff("").is_err());
}

#[test]
fn rejects_non_structure_root() {
    let err = parse_jff("<not-structure><type>fa</type></not-structure>").unwrap_err();
    assert!(err.contains("<structure>"));
}

#[test]
fn fa_type_is_parsed_as_automaton() {
    let xml = r#"<?xml version="1.0"?><structure>
        <type>fa</type>
        <automaton>
            <state id="0" name="q0"><x>0.0</x><y>0.0</y><initial/></state>
        </automaton>
    </structure>"#;

    match parse_jff(xml).unwrap() {
        JffParsed::Automaton(AutomatonType::FA, automaton) => assert_eq!(automaton.states.len(), 1),
        other => panic!("fa должен разбираться как автомат FA: {:?}", other),
    }
}

#[test]
fn mealy_type_is_parsed_as_automaton() {
    let xml = r#"<?xml version="1.0"?><structure>
        <type>mealy</type>
        <automaton>
            <state id="0" name="q0"><x>0.0</x><y>0.0</y><initial/></state>
            <transition><from>0</from><to>0</to><read>a</read><transout>1</transout></transition>
        </automaton>
    </structure>"#;

    match parse_jff(xml).unwrap() {
        JffParsed::Automaton(AutomatonType::MEALY, automaton) => {
            assert_eq!(automaton.states.len(), 1);
            assert_eq!(automaton.transitions.len(), 1);
            assert_eq!(automaton.transitions[0].symbol, 'a');
            assert_eq!(automaton.transitions[0].output, Some('1'));
        }
        other => panic!("mealy должен разбираться как автомат MEALY: {:?}", other),
    }
}

#[test]
fn non_automaton_types_return_grammar_mock() {
    for kind in ["grammar", "turing", "pda", ""] {
        let xml = format!(
            r#"<?xml version="1.0"?><structure><type>{}</type></structure>"#,
            kind
        );
        assert_grammar_mock(&xml);
    }
}

#[test]
fn grammar_mock_ignores_automaton_content() {
    let xml = r#"<?xml version="1.0"?><structure>
        <type>grammar</type>
        <automaton>
            <state id="0" name="q0"><x>0.0</x><y>0.0</y><initial/></state>
        </automaton>
    </structure>"#;

    assert_grammar_mock(xml);
}
