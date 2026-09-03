use std::fs;
use std::path::Path;

use crate::jff::{infer_kind, is_deterministic, parse_jff, to_jff};
use crate::structs::data_models::{AutomatonData, AutomatonKind, StateData, TransitionData};

const EVEN_A_JFF: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?><!--Created with JFLAP 7.1.--><structure>&#13;
	<type>fa</type>&#13;
	<automaton>&#13;
		<!--The list of states.-->&#13;
		<state id="0" name="q0">&#13;
			<x>154.0</x>&#13;
			<y>136.0</y>&#13;
			<initial/>&#13;
			<final/>&#13;
		</state>&#13;
		<state id="1" name="q1">&#13;
			<x>308.0</x>&#13;
			<y>147.0</y>&#13;
		</state>&#13;
		<!--The list of transitions.-->&#13;
		<transition>&#13;
			<from>0</from>&#13;
			<to>1</to>&#13;
			<read>a</read>&#13;
		</transition>&#13;
		<transition>&#13;
			<from>1</from>&#13;
			<to>0</to>&#13;
			<read>a</read>&#13;
		</transition>&#13;
		<transition>&#13;
			<from>0</from>&#13;
			<to>0</to>&#13;
			<read>b</read>&#13;
		</transition>&#13;
		<transition>&#13;
			<from>1</from>&#13;
			<to>1</to>&#13;
			<read>b</read>&#13;
		</transition>&#13;
	</automaton>&#13;
</structure>"#;

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
                isInitial: true,
                isFinal: true,
            },
            StateData {
                id: 1,
                label: "q1".to_string(),
                x: 308.0,
                y: 147.0,
                isInitial: false,
                isFinal: false,
            },
        ],
        transitions: vec![
            TransitionData {
                id: 1,
                from: 0,
                to: 1,
                symbol: 'a',
            },
            TransitionData {
                id: 2,
                from: 1,
                to: 0,
                symbol: 'a',
            },
            TransitionData {
                id: 3,
                from: 0,
                to: 0,
                symbol: 'b',
            },
            TransitionData {
                id: 4,
                from: 1,
                to: 1,
                symbol: 'b',
            },
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
        symbol: '$',
    });

    let xml = to_jff(&data);
    assert!(xml.contains("\t\t\t<read></read>"));
    assert!(!xml.contains("<read>$</read>"));
}

#[test]
fn escapes_special_characters() {
    let mut data = even_a_data();
    data.states[0].label = "q&<0>\"'".to_string();
    data.transitions[0].symbol = '&';

    let xml = to_jff(&data);
    assert!(xml.contains("name=\"q&amp;&lt;0&gt;&quot;&apos;\""));
    assert!(xml.contains("<read>&amp;</read>"));
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
        symbol: '$',
    });
    let nfa = to_jff(&nfa_data);

    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("jff_examples");
    fs::create_dir_all(&out_dir).unwrap();
    fs::write(out_dir.join("even_a_dfa.jff"), dfa).unwrap();
    fs::write(out_dir.join("nfa_with_epsilon.jff"), nfa).unwrap();

    let content = fs::read_to_string(out_dir.join("even_a_dfa.jff")).unwrap();
    let parsed = parse_jff(&content).unwrap();
    assert_eq!(parsed.states.len(), 2);
    assert!(parsed.states[0].isInitial);
    assert_eq!(parsed.transitions.len(), 4);
    assert_eq!(
        infer_kind(&parsed.states, &parsed.transitions),
        AutomatonKind::DFA
    );
}

#[test]
fn parses_even_a_jff() {
    let parsed = parse_jff(EVEN_A_JFF).unwrap();

    assert_eq!(parsed.kind, "fa");
    assert_eq!(parsed.states.len(), 2);
    assert_eq!(parsed.states[0].id, 0);
    assert_eq!(parsed.states[0].label, "q0");
    assert_eq!(parsed.states[0].x, 154.0);
    assert_eq!(parsed.states[0].y, 136.0);
    assert!(parsed.states[0].isInitial);
    assert!(parsed.states[0].isFinal);
    assert_eq!(parsed.states[1].label, "q1");
    assert!(!parsed.states[1].isInitial);
    assert!(!parsed.states[1].isFinal);

    assert_eq!(parsed.transitions.len(), 4);
    assert_eq!(
        parsed
            .transitions
            .iter()
            .map(|t| (t.from, t.to, t.symbol))
            .collect::<Vec<_>>(),
        vec![(0, 1, 'a'), (1, 0, 'a'), (0, 0, 'b'), (1, 1, 'b')]
    );

    assert_eq!(parsed.alphabet, vec!['a', 'b']);
}

#[test]
fn parses_epsilon_transition_as_dollar() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?><structure>
        <type>fa</type>
        <automaton>
            <state id="0" name="q0"><x>0.0</x><y>0.0</y><initial/></state>
            <state id="1" name="q1"><x>10.0</x><y>0.0</y></state>
            <transition><from>0</from><to>1</to><read></read></transition>
            <transition><from>0</from><to>1</to><read>a</read></transition>
        </automaton>
    </structure>"#;

    let parsed = parse_jff(xml).unwrap();
    assert_eq!(parsed.transitions.len(), 2);
    assert_eq!(parsed.transitions[0].symbol, '$');
    assert_eq!(parsed.transitions[1].symbol, 'a');
    assert_eq!(parsed.alphabet, vec!['a']);
}

#[test]
fn missing_read_is_epsilon() {
    let xml = r#"<?xml version="1.0"?><structure>
        <type>fa</type>
        <automaton>
            <state id="0" name="q0"><x>0.0</x><y>0.0</y><initial/></state>
            <transition><from>0</from><to>0</to></transition>
        </automaton>
    </structure>"#;

    let parsed = parse_jff(xml).unwrap();
    assert_eq!(parsed.transitions[0].symbol, '$');
}

#[test]
fn state_name_defaults_to_q_prefix() {
    let xml = r#"<?xml version="1.0"?><structure>
        <type>fa</type>
        <automaton>
            <state id="5"><x>1.0</x><y>2.0</y><final/></state>
        </automaton>
    </structure>"#;

    let parsed = parse_jff(xml).unwrap();
    assert_eq!(parsed.states[0].label, "q5");
    assert!(parsed.states[0].isFinal);
    assert_eq!(parsed.states[0].x, 1.0);
    assert_eq!(parsed.states[0].y, 2.0);
}

#[test]
fn decodes_xml_entities_in_name_and_read() {
    let xml = r#"<?xml version="1.0"?><structure>
        <type>fa</type>
        <automaton>
            <state id="0" name="q&amp;&lt;1&gt;"><x>0.0</x><y>0.0</y><initial/></state>
            <transition><from>0</from><to>0</to><read>&amp;</read></transition>
        </automaton>
    </structure>"#;

    let parsed = parse_jff(xml).unwrap();
    assert_eq!(parsed.states[0].label, "q&<1>");
    assert_eq!(parsed.transitions[0].symbol, '&');
    assert_eq!(parsed.alphabet, vec!['&']);
}

#[test]
fn rejects_non_fa_type() {
    let xml = r#"<?xml version="1.0"?><structure>
        <type>turing</type>
        <automaton></automaton>
    </structure>"#;

    let err = parse_jff(xml).unwrap_err();
    assert!(err.contains("turing"));
}

#[test]
fn rejects_invalid_xml() {
    assert!(parse_jff("not xml at all <").is_err());
    assert!(parse_jff("").is_err());
}

#[test]
fn rejects_transition_symbol_with_multiple_chars() {
    let xml = r#"<?xml version="1.0"?><structure>
        <type>fa</type>
        <automaton>
            <state id="0" name="q0"><x>0.0</x><y>0.0</y><initial/></state>
            <transition><from>0</from><to>0</to><read>ab</read></transition>
        </automaton>
    </structure>"#;

    assert!(parse_jff(xml).is_err());
}

#[test]
fn roundtrip_to_jff_then_parse() {
    let original = even_a_data();
    let xml = to_jff(&original);
    let parsed = parse_jff(&xml).unwrap();

    assert_eq!(parsed.kind, "fa");
    assert_eq!(parsed.states.len(), original.states.len());
    for (ps, os) in parsed.states.iter().zip(original.states.iter()) {
        assert_eq!(ps.id, os.id);
        assert_eq!(ps.label, os.label);
        assert_eq!(ps.x, os.x);
        assert_eq!(ps.y, os.y);
        assert_eq!(ps.isInitial, os.isInitial);
        assert_eq!(ps.isFinal, os.isFinal);
    }
    assert_eq!(parsed.alphabet, original.alphabet);
    assert_eq!(parsed.transitions.len(), original.transitions.len());
    for (pt, ot) in parsed.transitions.iter().zip(original.transitions.iter()) {
        assert_eq!(pt.from, ot.from);
        assert_eq!(pt.to, ot.to);
        assert_eq!(pt.symbol, ot.symbol);
    }
}

#[test]
fn infer_kind_for_even_a_is_dfa() {
    let parsed = parse_jff(EVEN_A_JFF).unwrap();
    assert!(is_deterministic(&parsed.states, &parsed.transitions));
    assert_eq!(
        infer_kind(&parsed.states, &parsed.transitions),
        AutomatonKind::DFA
    );
}

#[test]
fn infer_kind_with_epsilon_is_nfa() {
    let xml = r#"<?xml version="1.0"?><structure>
        <type>fa</type>
        <automaton>
            <state id="0" name="q0"><x>0.0</x><y>0.0</y><initial/></state>
            <state id="1" name="q1"><x>10.0</x><y>0.0</y><final/></state>
            <transition><from>0</from><to>1</to><read></read></transition>
        </automaton>
    </structure>"#;

    let parsed = parse_jff(xml).unwrap();
    assert!(!is_deterministic(&parsed.states, &parsed.transitions));
    assert_eq!(
        infer_kind(&parsed.states, &parsed.transitions),
        AutomatonKind::NFA
    );
}

#[test]
fn infer_kind_nondeterministic_is_nfa() {
    let xml = r#"<?xml version="1.0"?><structure>
        <type>fa</type>
        <automaton>
            <state id="0" name="q0"><x>0.0</x><y>0.0</y><initial/></state>
            <state id="1" name="q1"><x>10.0</x><y>0.0</y><final/></state>
            <transition><from>0</from><to>0</to><read>a</read></transition>
            <transition><from>0</from><to>1</to><read>a</read></transition>
        </automaton>
    </structure>"#;

    let parsed = parse_jff(xml).unwrap();
    assert!(!is_deterministic(&parsed.states, &parsed.transitions));
    assert_eq!(
        infer_kind(&parsed.states, &parsed.transitions),
        AutomatonKind::NFA
    );
}
