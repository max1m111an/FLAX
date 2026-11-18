//! Модульные тесты сериализации и разбора JFLAP XML (`.jff`) для машин Мили.

use crate::core::jff::{JffParsed, parse_jff};
use crate::core::types::{AutomatonData, AutomatonType, StateData, TransitionData};
use crate::mealy::jff::to_jff;

fn mealy_data() -> AutomatonData {
    AutomatonData {
        id: 1,
        name: "Mealy".to_string(),
        automatonType: AutomatonType::MEALY,
        states: vec![
            StateData {
                id: 0,
                label: "q0".to_string(),
                x: 100.0,
                y: 200.0,
                isInitial: true,
                isFinal: None,
            },
            StateData {
                id: 1,
                label: "q1".to_string(),
                x: 300.0,
                y: 200.0,
                isInitial: false,
                isFinal: None,
            },
        ],
        transitions: vec![
            TransitionData {
                id: 10,
                from: 0,
                to: 1,
                symbol: 'a',
                output: Some('1'),
            },
            TransitionData {
                id: 11,
                from: 1,
                to: 0,
                symbol: 'b',
                output: Some('0'),
            },
        ],
    }
}

#[test]
fn starts_with_xml_declaration() {
    let xml = to_jff(&mealy_data());
    assert!(xml.starts_with(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><!--Created with JFLAP 7.1.--><structure>"
    ));
}

#[test]
fn uses_mealy_type_in_serialization() {
    let xml = to_jff(&mealy_data());
    assert!(xml.contains("\t<type>mealy</type>\n"));
}

#[test]
fn serializes_states_without_final_flag() {
    let mut data = mealy_data();
    data.states[0].isFinal = Some(true);

    let xml = to_jff(&data);
    assert!(xml.contains("<state id=\"0\" name=\"q0\">"));
    assert!(xml.contains("<state id=\"1\" name=\"q1\">"));
    assert!(xml.contains("\t\t\t<x>100.0</x>"));
    assert!(xml.contains("\t\t\t<y>200.0</y>"));
    assert!(xml.contains("\t\t\t<initial/>\n"));
    assert!(!xml.contains("<final/>"));
}

#[test]
fn serializes_input_and_output_symbols() {
    let xml = to_jff(&mealy_data());

    assert_eq!(xml.matches("<transition>").count(), 2);
    assert!(xml.contains(
        "\t\t\t<from>0</from>\n\t\t\t<to>1</to>\n\t\t\t<read>a</read>\n\t\t\t<transout>1</transout>"
    ));
    assert!(xml.contains(
        "\t\t\t<from>1</from>\n\t\t\t<to>0</to>\n\t\t\t<read>b</read>\n\t\t\t<transout>0</transout>"
    ));
}

#[test]
fn escapes_special_characters() {
    let mut data = mealy_data();
    data.states[0].label = "q&<0>\"'".to_string();
    data.transitions[0].symbol = '&';
    data.transitions[0].output = Some('<');

    let xml = to_jff(&data);
    assert!(xml.contains("name=\"q&amp;&lt;0&gt;&quot;&apos;\""));
    assert!(xml.contains("<read>&amp;</read>"));
    assert!(xml.contains("<transout>&lt;</transout>"));
}

#[test]
fn missing_output_is_written_as_empty_transout() {
    let mut data = mealy_data();
    data.transitions[0].output = None;

    let xml = to_jff(&data);
    assert!(xml.contains("<transout></transout>"));
}

#[test]
fn parses_mealy_jff_with_input_and_output_symbols() {
    let xml = to_jff(&mealy_data());

    match parse_jff(&xml).unwrap() {
        JffParsed::Automaton(AutomatonType::MEALY, automaton) => {
            assert_eq!(automaton.states.len(), 2);
            assert!(automaton.states[0].isInitial);
            assert_eq!(automaton.transitions.len(), 2);
            assert_eq!(
                automaton
                    .transitions
                    .iter()
                    .map(|t| (t.from, t.to, t.symbol, t.output))
                    .collect::<Vec<_>>(),
                vec![(0, 1, 'a', Some('1')), (1, 0, 'b', Some('0'))]
            );
        }
        other => panic!("ожидался автомат MEALY, получено {:?}", other),
    }
}

#[test]
fn missing_transout_is_rejected() {
    let xml = r#"<?xml version="1.0"?><structure>
        <type>mealy</type>
        <automaton>
            <state id="0" name="q0"><x>0.0</x><y>0.0</y><initial/></state>
            <transition><from>0</from><to>0</to><read>a</read></transition>
        </automaton>
    </structure>"#;

    let err = parse_jff(xml).unwrap_err();
    assert!(err.contains("Выходной символ"));
}

#[test]
fn empty_transout_is_rejected() {
    let xml = r#"<?xml version="1.0"?><structure>
        <type>mealy</type>
        <automaton>
            <state id="0" name="q0"><x>0.0</x><y>0.0</y><initial/></state>
            <transition><from>0</from><to>0</to><read>a</read><transout></transout></transition>
        </automaton>
    </structure>"#;

    assert!(parse_jff(xml).is_err());
}

#[test]
fn empty_read_is_rejected() {
    let xml = r#"<?xml version="1.0"?><structure>
        <type>mealy</type>
        <automaton>
            <state id="0" name="q0"><x>0.0</x><y>0.0</y><initial/></state>
            <transition><from>0</from><to>0</to><read></read><transout>1</transout></transition>
        </automaton>
    </structure>"#;

    let err = parse_jff(xml).unwrap_err();
    assert!(err.contains("Входной символ"));
}

#[test]
fn multi_char_input_and_output_are_rejected() {
    let multi_input = r#"<?xml version="1.0"?><structure>
        <type>mealy</type>
        <automaton>
            <state id="0" name="q0"><x>0.0</x><y>0.0</y><initial/></state>
            <transition><from>0</from><to>0</to><read>ab</read><transout>1</transout></transition>
        </automaton>
    </structure>"#;
    assert!(parse_jff(multi_input).is_err());

    let multi_output = r#"<?xml version="1.0"?><structure>
        <type>mealy</type>
        <automaton>
            <state id="0" name="q0"><x>0.0</x><y>0.0</y><initial/></state>
            <transition><from>0</from><to>0</to><read>a</read><transout>01</transout></transition>
        </automaton>
    </structure>"#;
    assert!(parse_jff(multi_output).is_err());
}

#[test]
fn no_states_and_transitions_yields_empty_automaton() {
    let data = AutomatonData {
        id: 2,
        name: "Empty".to_string(),
        automatonType: AutomatonType::MEALY,
        states: Vec::new(),
        transitions: Vec::new(),
    };

    let xml = to_jff(&data);
    assert!(xml.contains("<automaton>"));
    assert!(xml.contains("</automaton>"));
    assert!(!xml.contains("<state"));
    assert!(!xml.contains("<transition>"));
    assert!(xml.contains("</structure>"));
}
