//! Тесты выбора домена по имени IPC-команды.

use super::{Route, route};

#[test]
fn fa_commands_route_to_fa() {
    assert_eq!(route("fa_create_new"), Route::Fa);
    assert_eq!(route("fa_add_state"), Route::Fa);
    assert_eq!(route("fa_remove_automaton"), Route::Fa);
}

#[test]
fn mealy_commands_route_to_mealy() {
    assert_eq!(route("mealy_create_new"), Route::Mealy);
    assert_eq!(route("mealy_multi_run"), Route::Mealy);
    assert_eq!(route("mealy_generate_inputs"), Route::Mealy);
}

#[test]
fn other_commands_route_to_core() {
    assert_eq!(route("load_file"), Route::Core);
    assert_eq!(route("save_file"), Route::Core);
    assert_eq!(route("unknown_command"), Route::Core);
}

#[test]
fn partial_prefixes_are_not_mistaken_for_domains() {
    assert_eq!(route("fa"), Route::Core);
    assert_eq!(route("faq"), Route::Core);
    assert_eq!(route("mealy"), Route::Core);
    assert_eq!(route("mealyish"), Route::Core);
}
