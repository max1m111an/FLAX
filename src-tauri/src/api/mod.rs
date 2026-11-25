//! Маршрутизация IPC-команд по доменам: каждый домен регистрирует свои команды
//! отдельным обработчиком, а общий диспетчер передаёт вызов нужному из них.

use tauri::ipc::Invoke;

#[cfg(test)]
mod tests;

/// Домен, к которому относится команда, по её префиксу.
#[derive(Debug, PartialEq, Eq)]
enum Route {
    Core,
    Fa,
    Mealy,
}

/// Определяет домен по имени команды: `fa_*` и `mealy_*` разносятся по своим
/// модулям, все остальные команды считаются общими (`core`).
fn route(command: &str) -> Route {
    if command.starts_with("fa_") {
        Route::Fa
    } else if command.starts_with("mealy_") {
        Route::Mealy
    } else {
        Route::Core
    }
}

/// Принимает IPC-вызов и передаёт его обработчику нужного домена.
///
/// Возвращает `true`, если команда обработана, и `false`, если ни один
/// обработчик её не распознал (Tauri считает такую команду неизвестной).
pub(crate) fn handler(invoke: Invoke) -> bool {
    let domain = {
        let command = invoke.message.command();
        route(command)
    };

    match domain {
        Route::Core => crate::core::api::handler(invoke),
        Route::Fa => crate::fa::api::handler(invoke),
        Route::Mealy => crate::mealy::api::handler(invoke),
    }
}
