use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

use rand::Rng;
use tauri::{State, http::StatusCode};

use crate::core::automaton::AutomatonStore;
use crate::core::jff::{JffParsed, parse_jff};
use crate::core::types::{
    AutomatonData,
    AutomatonType,
    EPSILON,
    GenerateInputsResult,
    LoadResult,
    LoadedFile,
    MealyMultiRunResult,
    MealyRunResult,
    MultiRunResult,
    RunResult,
    StateData,
    StatusResult,
    TransitionData,
    alphabet_from_transitions,
    //TransitionResult, OperationResult, StateResult
};

pub const MAX_ID: i32 = 10_000_000;

pub fn generate_id(used: &HashSet<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    loop {
        let id = rng.gen_range(1..=MAX_ID);
        if !used.contains(&id) {
            return id;
        }
    }
}

/// Загружает JFLAP-файл и регистрирует объект в хранилище.
#[tauri::command]
pub fn load_file(state: State<'_, AutomatonStore>, path: String) -> LoadResult {
    match load_via_file_with(&state, &path, "Автомат", |content| {
        let (is_automaton, automaton_type, states, transitions) = match parse_jff(content)? {
            JffParsed::Automaton(automaton_type, automaton) => (
                true,
                automaton_type,
                automaton.states,
                automaton.transitions,
            ),
            JffParsed::Grammar(_) => (false, AutomatonType::FA, Vec::new(), Vec::new()),
        };
        Ok((
            is_automaton,
            AutomatonData {
                id: 0,
                name: String::new(),
                automatonType: automaton_type,
                states,
                transitions,
            },
        ))
    }) {
        Ok(loaded) => LoadResult {
            status: StatusCode::OK,
            message: if loaded.isAutomaton {
                "Автомат загружен из файла".to_string()
            } else {
                "Формальный язык загружен из файла (заглушка)".to_string()
            },
            isAutomaton: Some(loaded.isAutomaton),
            data: Some(loaded.data),
        },
        Err(message) => LoadResult {
            status: StatusCode::BAD_REQUEST,
            message,
            isAutomaton: None,
            data: None,
        },
    }
}

/// Загружает формальный объект из файла и регистрирует его в хранилище.
///
/// Разбор содержимого делегируется `parse`: он должен вернуть признак
/// автомата, прочитанный из JFLAP-элемента `<type>`, и общие данные. Так
/// `core` не зависит от конкретного формата (FA, Mealy, грамматика, ...), а
/// новые формальные языки подключаются через собственный парсер.
pub(crate) fn load_via_file_with<F>(
    store: &AutomatonStore,
    path: &str,
    default_name: &str,
    parse: F,
) -> Result<LoadedFile, String>
where
    F: FnOnce(&str) -> Result<(bool, AutomatonData), String>,
{
    let content =
        fs::read_to_string(path).map_err(|err| format!("Не удалось прочитать файл: {}", err))?;

    let (is_automaton, mut data) = parse(&content)?;

    data.id = 0;
    data.name = Path::new(path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| default_name.to_string());

    let data = store.insert(data);

    Ok(LoadedFile {
        isAutomaton: is_automaton,
        data,
    })
}

/// Универсальный эндпоинт сохранения: берёт объект из хранилища и пишет его
/// в файл в формате, соответствующем типу автомата.
#[tauri::command]
pub fn save_file(
    state: State<'_, AutomatonStore>,
    automaton_id: i32,
    path: String,
) -> StatusResult {
    let entry = match check_automaton_status(state.inner(), automaton_id) {
        Ok(e) => e,
        Err(e) => return e,
    };

    save_automaton(&entry, &path)
}

/// Сериализует автомат в формате его типа и записывает результат в `path`.
/// Новый тип автомата подключается добавлением ветки в `match`.
pub(crate) fn save_automaton(entry: &AutomatonData, path: &str) -> StatusResult {
    let content = match entry.automatonType {
        AutomatonType::FA => crate::fa::jff::to_jff(entry),
        AutomatonType::MEALY => crate::mealy::jff::to_jff(entry),
    };

    save_file_with(path, &content)
}

/// Записывает готовое содержимое в файл, инкапсулируя работу с файловой
/// системой и формирование ответа об успехе/ошибке.
pub(crate) fn save_file_with(path: &str, content: &str) -> StatusResult {
    match fs::write(path, content) {
        Ok(()) => StatusResult {
            status: StatusCode::OK,
            message: format!("Автомат сохранён в {}", path),
        },
        Err(err) => StatusResult {
            status: StatusCode::BAD_REQUEST,
            message: format!("Не удалось сохранить файл: {}", err),
        },
    }
}

/// Общий драйвер пакетного прогона: строит автомат один раз (`build`) и
/// прогоняет по нему все строки (`run_line`), сохраняя порядок `inputs`.
///
/// `core` ничего не знает о конкретном типе автомата и формате ответа:
/// построение (`data_to_fa` / `data_to_mm`) и обработка одной строки
/// передаются замыканиями, а ошибка построения пробрасывается как `Err`.
pub(crate) fn multi_run_automaton<A, T, B, R>(
    data: &AutomatonData,
    inputs: &[String],
    build: B,
    run_line: R,
) -> Result<Vec<T>, String>
where
    B: FnOnce(&[StateData], &[TransitionData]) -> Result<A, String>,
    R: Fn(&A, &str) -> T,
{
    let automaton = build(&data.states, &data.transitions)?;
    Ok(inputs
        .iter()
        .map(|line| run_line(&automaton, line))
        .collect())
}

/// Инкапсулированная проверка существования автомата.
/// При отсутствии возвращает переданный `err_resp`, что позволяет
/// использовать функцию с разными типами ответов команд.
///
/// usage: `let entry = check_automaton(store, automaton_id, err_resp)?;`
#[warn(dead_code)]
pub(crate) fn check_automaton<T>(
    store: &AutomatonStore,
    automaton_id: i32,
    err_resp: T,
) -> Result<AutomatonData, T> {
    store.get(automaton_id).ok_or(err_resp)
}

/// Проверка существования автомата с шаблоном ошибки [`StatusResult`].
#[warn(dead_code)]
pub(crate) fn check_automaton_status(
    store: &AutomatonStore,
    automaton_id: i32,
) -> Result<AutomatonData, StatusResult> {
    check_automaton(
        store,
        automaton_id,
        StatusResult {
            status: StatusCode::NOT_FOUND,
            message: format!("Автомат с id {} не найден", automaton_id),
        },
    )
}

/// Проверка существования автомата с шаблоном ошибки [`StateResult`].
//pub(crate) fn check_automaton_state(
//    store: &AutomatonStore,
//    automaton_id: i32,
//) -> Result<AutomatonData, StateResult> {
//    check_automaton(
//        store,
//        automaton_id,
//        StateResult {
//            status: StatusCode::NOT_FOUND,
//            message: format!("Автомат с id {} не найден", automaton_id),
//            state: None,
//        },
//    )
//}

/// Проверка существования автомата с шаблоном ошибки [`TransitionResult`].
//#[warn(dead_code)]
//pub(crate) fn check_automaton_transition(
//    store: &AutomatonStore,
//    automaton_id: i32,
//) -> Result<AutomatonData, TransitionResult> {
//    check_automaton(
//        store,
//        automaton_id,
//        TransitionResult {
//            status: StatusCode::NOT_FOUND,
//            message: format!("Автомат с id {} не найден", automaton_id),
//            transition: Vec::new(),
//        },
//    )
//}

/// Проверка существования автомата с шаблоном ошибки [`RunResult`].
#[warn(dead_code)]
pub(crate) fn check_automaton_run(
    store: &AutomatonStore,
    automaton_id: i32,
) -> Result<AutomatonData, RunResult> {
    check_automaton(
        store,
        automaton_id,
        RunResult {
            status: StatusCode::NOT_FOUND,
            message: format!("Автомат с id {} не найден", automaton_id),
            traces: Vec::new(),
        },
    )
}

/// Проверка существования автомата с шаблоном ошибки [`MultiRunResult`].
#[warn(dead_code)]
pub(crate) fn check_automaton_multi_run(
    store: &AutomatonStore,
    automaton_id: i32,
) -> Result<AutomatonData, MultiRunResult> {
    check_automaton(
        store,
        automaton_id,
        MultiRunResult {
            status: StatusCode::NOT_FOUND,
            message: format!("Автомат с id {} не найден", automaton_id),
            traces: Vec::new(),
        },
    )
}

/// Проверка существования автомата с шаблоном ошибки [`MealyRunResult`].
#[warn(dead_code)]
pub(crate) fn check_automaton_mealy_run(
    store: &AutomatonStore,
    automaton_id: i32,
) -> Result<AutomatonData, MealyRunResult> {
    check_automaton(
        store,
        automaton_id,
        MealyRunResult {
            status: StatusCode::NOT_FOUND,
            message: format!("Автомат с id {} не найден", automaton_id),
            output: String::new(),
            steps: Vec::new(),
        },
    )
}

/// Проверка существования автомата с шаблоном ошибки [`MealyMultiRunResult`].
#[warn(dead_code)]
pub(crate) fn check_automaton_mealy_multi_run(
    store: &AutomatonStore,
    automaton_id: i32,
) -> Result<AutomatonData, MealyMultiRunResult> {
    check_automaton(
        store,
        automaton_id,
        MealyMultiRunResult {
            status: StatusCode::NOT_FOUND,
            message: format!("Автомат с id {} не найден", automaton_id),
            results: Vec::new(),
        },
    )
}

/// Проверка существования автомата с шаблоном ошибки [`OperationResult`].
//#[warn(dead_code)]
//pub(crate) fn check_automaton_operation(
//    store: &AutomatonStore,
//    automaton_id: i32,
//) -> Result<AutomatonData, OperationResult> {
//    check_automaton(
//        store,
//        automaton_id,
//        OperationResult {
//            status: StatusCode::NOT_FOUND,
//            message: format!("Автомат с id {} не найден", automaton_id),
//            automaton: None,
//        },
//    )
//}

/// Проверка существования автомата с шаблоном ошибки [`GenerateInputsResult`].
#[warn(dead_code)]
pub(crate) fn check_automaton_generate_inputs(
    store: &AutomatonStore,
    automaton_id: i32,
) -> Result<AutomatonData, GenerateInputsResult> {
    check_automaton(
        store,
        automaton_id,
        GenerateInputsResult {
            status: StatusCode::NOT_FOUND,
            message: format!("Автомат с id {} не найден", automaton_id),
            inputs: Vec::new(),
        },
    )
}

/// Символы вне входного алфавита для негативных тестов. Сначала цифры, затем
/// буквы; `eps` включается всегда, чтобы проверить отказ от `$` как входа.
fn outside_alphabet_symbols(alphabet: &HashSet<char>) -> Vec<char> {
    let mut out = Vec::new();
    let preferred: Vec<char> = (b'0'..=b'9')
        .chain(b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .map(|b| b as char)
        .collect();
    let mut candidates: Vec<char> = vec!['0', '4', EPSILON];
    candidates.extend(preferred);
    for c in candidates {
        if !alphabet.contains(&c) && !out.contains(&c) {
            out.push(c);
            if out.len() == 2 {
                break;
            }
        }
    }
    out
}

/// Кратчайший непустой цикл из `start` обратно в `start` в детерминированном
/// графе переходов. Поиск в ширину гарантирует минимальную длину цикла.
fn shortest_cycle(
    start: i32,
    transitions: &HashMap<(i32, char), i32>,
    alphabet: &[char],
    depth: usize,
) -> Option<String> {
    let mut queue: VecDeque<(i32, String, usize)> = VecDeque::new();
    let mut visited: HashSet<i32> = HashSet::new();

    for &ch in alphabet {
        if let Some(&to) = transitions.get(&(start, ch)) {
            if to == start {
                return Some(ch.to_string());
            }
            if visited.insert(to) {
                queue.push_back((to, ch.to_string(), 1));
            }
        }
    }

    while let Some((state, path, len)) = queue.pop_front() {
        if len >= depth {
            continue;
        }
        for &ch in alphabet {
            if let Some(&to) = transitions.get(&(state, ch)) {
                if to == start {
                    let mut cycle = path;
                    cycle.push(ch);
                    return Some(cycle);
                }
                if visited.insert(to) {
                    let mut next = path.clone();
                    next.push(ch);
                    queue.push_back((to, next, len + 1));
                }
            }
        }
    }
    None
}

/// Генерирует упорядоченные тестовые входы для детерминированного автомата
/// (машина Мили, в будущем — машина Мура). В отличие от КА, здесь нет
/// eps-переходов и финальных состояний, зато важно покрыть выходные символы:
/// поэтому для каждого выхода берётся кратчайший переход, его порождающий.
///
/// Негативные сценарии (`$` и символы вне алфавита) добавляются как входные
/// строки — они не должны быть переходами самой машины.
pub(crate) fn generate_test_inputs_deterministic(
    entry: &AutomatonData,
    cap: usize,
    depth: usize,
) -> Result<Vec<String>, String> {
    let initial = entry
        .states
        .iter()
        .find(|s| s.isInitial)
        .ok_or_else(|| "Начальное состояние не задано".to_string())?
        .id;
    let alphabet = alphabet_from_transitions(&entry.transitions);
    let alphabet_set: HashSet<char> = alphabet.iter().copied().collect();
    let state_ids: HashSet<i32> = entry.states.iter().map(|s| s.id).collect();

    // Осиротевшие переходы и `$` игнорируются: это не корректные входы машины.
    let mut transitions: HashMap<(i32, char), i32> = HashMap::new();
    let mut output_transitions: HashMap<char, Vec<(i32, char)>> = HashMap::new();
    for t in &entry.transitions {
        if t.symbol == EPSILON || !state_ids.contains(&t.from) || !state_ids.contains(&t.to) {
            continue;
        }
        transitions.insert((t.from, t.symbol), t.to);
        if let Some(output) = t.output {
            output_transitions
                .entry(output)
                .or_default()
                .push((t.from, t.symbol));
        }
    }

    // Кратчайшие пути до всех достижимых состояний.
    let mut dist: HashMap<i32, String> = HashMap::new();
    dist.insert(initial, String::new());
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(initial);
    while let Some(state) = queue.pop_front() {
        let path = dist.get(&state).cloned().unwrap_or_default();
        if path.chars().count() >= depth {
            continue;
        }
        for &ch in &alphabet {
            if let Some(&to) = transitions.get(&(state, ch)) {
                if !dist.contains_key(&to) {
                    let mut next = path.clone();
                    next.push(ch);
                    dist.insert(to, next);
                    queue.push_back(to);
                }
            }
        }
    }

    let mut states: Vec<i32> = dist.keys().copied().collect();
    states.sort_by_key(|s| dist.get(s).map(|p| p.chars().count()).unwrap_or(0));

    let mut result: HashSet<String> = HashSet::new();
    // Повторы цикла должны остаться в ответе даже при большом количестве
    // коротких вариантов и маленьком `cap`.
    let mut priority: HashSet<String> = HashSet::new();
    result.insert(String::new());
    for &ch in &alphabet {
        result.insert(ch.to_string());
    }
    // `$` недопустим как переход машины Мили, но должен присутствовать как
    // негативный тестовый вход.
    result.insert(EPSILON.to_string());
    let outside = outside_alphabet_symbols(&alphabet_set);

    for state in &states {
        if let Some(path) = dist.get(state) {
            result.insert(path.clone());
        }
    }

    // Покрытие каждого перехода: путь до состояния + символ перехода.
    for ((from, symbol), _) in &transitions {
        if let Some(path) = dist.get(from) {
            let mut activated = path.clone();
            activated.push(*symbol);
            result.insert(activated);
        }
    }

    // Покрытие каждого выходного символа (цель генерации — получить много
    // разных выходов при мультипрогоне).
    for candidates in output_transitions.values() {
        let best = candidates
            .iter()
            .filter(|(from, _)| dist.contains_key(from))
            .min_by_key(|(from, _)| {
                dist.get(from)
                    .map(|p| p.chars().count())
                    .unwrap_or(usize::MAX)
            });
        if let Some((from, symbol)) = best {
            let mut path = dist.get(from).cloned().unwrap_or_default();
            path.push(*symbol);
            result.insert(path);
        }
    }

    // Негативные случаи: путь до состояния, в котором нет перехода по символу
    // алфавита (прогон остановится до конца строки).
    let mut dead_end_states = states.clone();
    dead_end_states.truncate(8);
    for state in dead_end_states {
        let path = dist.get(&state).cloned().unwrap_or_default();
        for &ch in &alphabet {
            if !transitions.contains_key(&(state, ch)) {
                let mut stuck = path.clone();
                stuck.push(ch);
                result.insert(stuck);
            }
        }
    }

    // Всего 2–3 негативных строки: `$` уже добавлен выше, плюс две комбинации
    // «символ внутри алфавита + символ вне алфавита».
    let mut bases: Vec<String> = states.iter().filter_map(|s| dist.get(s)).cloned().collect();
    bases.sort();
    bases.dedup();
    bases.sort_by_key(|p| p.chars().count());
    let base = bases
        .iter()
        .find(|p| !p.is_empty())
        .or_else(|| bases.first());
    let invalid = outside
        .iter()
        .find(|c| **c != EPSILON)
        .map(|c| c.to_string())
        .unwrap_or_else(|| EPSILON.to_string());
    if let Some(base) = base {
        result.insert(format!("{}{}", invalid, base));
        result.insert(format!("{}{}", base, invalid));
    }

    // Только 2 и 3 прохода по циклу — без длинных цепочек из 20 символов.
    let mut cycle_input = None;
    for state in &states {
        if let Some(cycle) = shortest_cycle(*state, &transitions, &alphabet, depth) {
            let prefix = dist.get(state).cloned().unwrap_or_default();
            cycle_input = Some((prefix, cycle));
            break;
        }
    }
    if let Some((prefix, cycle)) = cycle_input {
        for count in 2..=3 {
            let mut input = prefix.clone();
            for _ in 0..count {
                input.push_str(&cycle);
            }
            priority.insert(input);
        }
    }

    let mut sorted: Vec<String> = result
        .into_iter()
        .filter(|s| !priority.contains(s))
        .collect();
    sorted.sort_by(|a, b| a.chars().count().cmp(&b.chars().count()).then(a.cmp(b)));
    let mut inputs: Vec<String> = sorted
        .into_iter()
        .take(cap.saturating_sub(priority.len()))
        .collect();
    inputs.extend(priority);
    inputs.sort_by(|a, b| a.chars().count().cmp(&b.chars().count()).then(a.cmp(b)));
    inputs.dedup();
    inputs.truncate(cap);
    Ok(inputs)
}

/// Генерирует входы для режима Multiple Run, чтобы пользователь мог сразу
/// увидеть множество выходных последовательностей машины Мили. Для КА используется
/// существующий генератор, для Мили — общий детерминированный генератор ядра.
/// При появлении `AutomatonType::MOORE` здесь добавится ещё одна ветка с тем же
/// генератором (источник выходных символов у Мура — состояния).
pub(crate) fn gen_multi_run_for_output(
    store: &AutomatonStore,
    automaton_id: i32,
) -> GenerateInputsResult {
    let entry = match check_automaton_generate_inputs(store, automaton_id) {
        Ok(entry) => entry,
        Err(err) => return err,
    };

    let inputs = match entry.automatonType {
        AutomatonType::FA => match crate::fa::api::data_to_fa(&entry.states, &entry.transitions) {
            Ok(fa) => fa.generate_test_inputs(50, 15),
            Err(_) => {
                return GenerateInputsResult {
                    status: StatusCode::BAD_REQUEST,
                    message: "Некорректный автомат".to_string(),
                    inputs: Vec::new(),
                };
            }
        },
        AutomatonType::MEALY => {
            if let Err(err) = crate::mealy::api::data_to_mm(&entry.states, &entry.transitions) {
                return GenerateInputsResult {
                    status: StatusCode::BAD_REQUEST,
                    message: format!("Некорректный автомат: {}", err),
                    inputs: Vec::new(),
                };
            }
            match generate_test_inputs_deterministic(&entry, 50, 15) {
                Ok(inputs) => inputs,
                Err(err) => {
                    return GenerateInputsResult {
                        status: StatusCode::BAD_REQUEST,
                        message: format!("Некорректный автомат: {}", err),
                        inputs: Vec::new(),
                    };
                }
            }
        }
    };

    GenerateInputsResult {
        status: StatusCode::OK,
        message: format!("Сгенерировано {} тестовых входов", inputs.len()),
        inputs,
    }
}
