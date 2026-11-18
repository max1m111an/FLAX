// Убирает лишнее окно консоли в Windows в релизной сборке, НЕ УДАЛЯТЬ!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    flax_lib::run()
}
