# AGENTS.md — FLAX (backend, `src-tauri`)

Руководство для ИИ-агентов, работающих с серверной частью FLAX.

## 1. О проекте

**FLAX (Formal Languages and Automata eXplorer)** — учебный пакет для моделирования
конечных автоматов и формальных языков (аналог JFLAP). Десктопное приложение на
**Tauri 2**: бэкенд на **Rust** (этот каталог), фронтенд на **React + TypeScript + Vite**
(каталог `../src`).

Бэкенд отвечает за:
- хранение автоматов в памяти (`AutomatonStore`);
- редактирование состояний и переходов;
- моделирование работы автоматов (КА / НКА с eps-переходами, машина Мили);
- генерацию тестовых входных цепочек;
- импорт/экспорт в формате JFLAP XML (`.jff`).

Поддерживаемые типы автоматов: `FA` (конечный автомат), `MEALY` (машина Мили).
Зарезервированы (закомментированы в `AutomatonType`): `MOORE`, `PDA`, `TURING`.

## 2. Границы рабочей области (ОБЯЗАТЕЛЬНО)

- **Рабочая директория агента — `src-tauri/`.** Изменять можно только файлы внутри неё.
- Если задача затрагивает файлы **вне** `src-tauri/` (фронтенд `../src`, в частности
  `../src/api/faAPI.ts`, `../src/api/MealyAPI.ts`, `../src/api/jffAPI.ts`; `../package.json`,
  `../Makefile`, `../scripts/`, `../.github/` и т. д.), агент **обязан явно упомянуть**,
  какие файлы и как нужно изменить, **но не изменять их**.
- Типичный случай: изменение сигнатуры Tauri-команды или DTO ломает контракт с
  фронтендом — агент должен перечислить затронутые вызовы `invoke(...)` во фронтенде.
- Не трогать сгенерированные/служебные каталоги: `target/`, `gen/`, `icons/`, `Cargo.lock`
  (последний — только через `cargo`, при реальном изменении зависимостей).

## 3. Технологии и сборка

| Что | Значение |
|---|---|
| Язык | Rust, `edition = "2024"` |
| Crate | бинарь `flax`, библиотека `flax_lib` (`staticlib`, `cdylib`, `rlib`) |
| Зависимости | `tauri 2`, `tauri-plugin-opener`, `tauri-plugin-dialog`, `tauri-plugin-fs`, `serde`/`serde_json`, `rand 0.8`, `roxmltree 0.21`, `http-serde 2` |
| Конфиг | `tauri.conf.json` (окно 1024×768, dev URL `http://localhost:1420`, frontendDist `../dist`) |
| Права | `capabilities/default.json` (core, opener, dialog, fs: чтение/запись текстовых файлов, запись в Desktop/Documents/Downloads) |

Команды (из `src-tauri/`):
```bash
cargo build            # сборка
cargo test             # все unit-тесты
cargo test mealy       # тесты по фильтру имени
cargo clippy           # линтер
cargo fmt              # форматирование
```
Из корня репозитория также есть `make test` / `make test ARGS=<фильтр>` (вне рабочей области — не менять).

## 4. Структура каталога

```
src-tauri/
├── build.rs                  # tauri_build
├── Cargo.toml
├── tauri.conf.json
├── capabilities/default.json
└── src/
    ├── main.rs               # точка входа → flax_lib::run()
    ├── lib.rs                # Builder Tauri: плагины, AutomatonStore, регистрация команд
    ├── core/                 # общий доменный слой
    │   ├── mod.rs
    │   ├── types.rs          # DTO, трейты автоматов, AutomatonStore, EPSILON
    │   ├── api.rs            # load_file, save_file, generate_id, общие драйверы
    │   ├── tests.rs
    │   └── jff/              # общая инфраструктура JFLAP XML
    │       ├── mod.rs        # XML-утилиты, parse_jff (диспетчер по <type>)
    │       ├── types.rs      # JffAutomaton, JffGrammar, JffParsed
    │       └── tests.rs
    ├── fa/                   # конечные автоматы
    │   ├── mod.rs
    │   ├── types.rs          # FA, FABuilder, алгоритмы моделирования/генерации
    │   ├── api.rs            # Tauri-команды fa_*
    │   ├── tests.rs
    │   └── jff/{mod.rs, tests.rs}   # to_jff / parse_automaton для FA
    └── mealy/                # машины Мили
        ├── mod.rs
        ├── types.rs          # MM, MealyTransition, MealyBuilder
        ├── api.rs            # Tauri-команды mealy_* и чистые хелперы
        ├── tests.rs
        └── jff/{mod.rs, tests.rs}   # to_jff / parse_automaton для Mealy
```

## 5. Модули

### 5.1. `lib.rs`
Создаёт `tauri::Builder`, подключает плагины `fs`, `dialog`, `opener`, регистрирует
состояние `AutomatonStore` через `.manage(...)` и все команды в `generate_handler![...]`.
**Каждая новая `#[tauri::command]` должна быть добавлена сюда.**

### 5.2. `core` — общий доменный слой
Не зависит от конкретного типа автомата (кроме диспетчеров `parse_jff` и `save_automaton`).

- **`core::types`** — контракт данных с фронтендом (serde DTO), трейты `Automaton` /
  `NondeterministicAutomaton`, реестр `AutomatonStore`, константа `EPSILON = '$'`,
  функция `alphabet_from_transitions`.
- **`core::api`**:
  - `MAX_ID = 10_000_000`; `generate_id(used: &HashSet<i32>) -> i32` — случайный уникальный id в `1..=MAX_ID`;
  - команды `load_file`, `save_file`;
  - `load_via_file_with(store, path, default_name, parse)` — чтение файла + делегированный разбор + регистрация в store (имя = имя файла без расширения);
  - `save_automaton(entry, path)` — выбор сериализатора по `automatonType` (`fa::jff::to_jff` / `mealy::jff::to_jff`);
  - `save_file_with(path, content)` — запись файла и формирование `StatusResult`;
  - `multi_run_automaton(data, inputs, build, run_line)` — общий драйвер пакетного прогона: строит автомат один раз, прогоняет все строки с сохранением порядка.
- **`core::jff`** — общая логика JFLAP XML:
  - утилиты: `escape`, `fmt_coord`, `element_text`, `parse_attr_i32`, `one_char`;
  - каркас документа: `document_header(kind)`, `document_footer()`, `write_transitions_header`;
  - состояния: `write_states`, `parse_states`;
  - переходы: `write_transition`, `write_transition_with_output`, `parse_transition_endpoints`, `transition_read`, `transition_output`, `parse_transitions(automaton, read)`;
  - `read_type(root)`, `find_automaton(root)`;
  - **`parse_jff(xml) -> Result<JffParsed, String>`** — проверяет корень `<structure>` и по `<type>` вызывает `fa::jff::parse_automaton` (`"fa"`) или `mealy::jff::parse_automaton` (`"mealy"`); прочие типы → заглушка `JffParsed::Grammar`.
  - Здесь только то, что одинаково для всех типов; специфика формата — в модулях типов.

### 5.3. `fa` — конечные автоматы (ДКА/НКА, eps-переходы)
- **`fa::types`** — `FA`, `FABuilder` (см. раздел 6), константа `MAX_THREADS = 1_000_000`
  (ограничение числа параллельных «потоков» при моделировании НКА).
  Ключевые методы `FA`: `new`, `builder`, `get_transitions`, `reachable_states`, `is_empty`,
  `is_deterministic`, `run(input) -> Option<Vec<RunStep>>` (поиск допускающего пути),
  `run_partial(input) -> (Vec<Trace>, bool)` (параллельное моделирование в стиле JFLAP
  с записью eps-шагов `$`), `generate_test_inputs(cap, depth) -> Vec<String>`.
  Реализует трейты `Automaton` и `NondeterministicAutomaton`.
- **`fa::api`** — Tauri-команды `fa_*` (раздел 7.2) и хелперы:
  `data_to_fa(states, transitions) -> Result<FA, String>` (DTO → `FA`, переходы с
  несуществующими состояниями игнорируются), `test_line(fa, input) -> (bool, usize)`,
  `multi_run_fa(entry, inputs) -> MultiRunResult`.
- **`fa::jff`** — `to_jff(data) -> String`, `parse_automaton(root) -> Result<JffAutomaton, String>`
  (пустой/отсутствующий `<read>` = eps `$`), `is_deterministic(states, transitions) -> bool`.

### 5.4. `mealy` — машины Мили
Детерминированный преобразователь: выход привязан к переходу; нет финальных состояний
и eps-переходов; для каждой пары `(state, input)` — не более одного перехода.
- **`mealy::types`** — `MM`, `MealyTransition`, `MealyBuilder` (раздел 6).
  Методы `MM`: `new`, `builder`, `states`, `input_alphabet`, `output_alphabet`,
  `initial_state`, `transitions`, `step(from, input) -> Option<(i32, char)>`,
  `run(input) -> (Vec<MealyRunStep>, String)` (останавливается на первом отсутствующем переходе).
- **`mealy::api`** — Tauri-команды `mealy_*` и чистые (тестируемые без Tauri) хелперы:
  `create_new_mealy`, `add_mealy_transition`, `update_mealy_transition`,
  `remove_mealy_transition` (валидация до мутации — при `Err` сущность не меняется),
  `data_to_mm`, `run_mealy`, `multi_run_mealy`.
  Команды над состояниями делегируют в `fa::api` (состояния общие), передавая `is_final = None`.
- **`mealy::jff`** — `to_jff` (`<type>mealy</type>`, вход в `<read>`, выход в `<transout>`,
  `<final/>` не пишется), `parse_automaton`.

## 6. Структуры данных

### 6.1. DTO (`core::types`) — контракт с фронтендом
Поля в camelCase (`#[allow(non_snake_case)]`) — **их имена являются частью API** (см. раздел 10, п. 7).

| Структура | Поля |
|---|---|
| `StateData` | `id: i32`, `label: String`, `x: f32`, `y: f32`, `isInitial: bool`, `isFinal: Option<bool>` (не сериализуется при `None`; у Мили всегда `None`) |
| `TransitionData` | `id: i32`, `from: i32`, `to: i32`, `symbol: char` (`'$'` = eps), `output: Option<char>` (только у Мили) |
| `AutomatonType` (enum) | `FA` (default), `MEALY` |
| `AutomatonData` | `id: i32`, `name: String`, `automatonType: AutomatonType`, `states: Vec<StateData>`, `transitions: Vec<TransitionData>`; метод `get_alphabet()` |
| `RunStep` | `from`, `symbol`, `to` |
| `Trace` | `steps: Vec<RunStep>`, `isFinal: bool` |
| `MealyRunStep` | `from`, `symbol`, `to`, `output: char` |
| `LineTest` | `line: String`, `isFinal: bool`, `correctSymbols: usize` |
| `MealyLineTest` | `line: String`, `output: String`, `isComplete: bool` |
| `LoadedFile` (внутр., не serde) | `isAutomaton: bool`, `data: AutomatonData` |

Алфавит **не хранится** отдельно: он всегда выводится из переходов
(`alphabet_from_transitions` — уникальные не-eps символы в порядке появления).

### 6.2. Ответы команд
Во всех ответах `status` — HTTP-код (`tauri::http::StatusCode`, сериализуется числом
через `http_serde::status_code`) и `message: String` (на русском).

| Ответ | Доп. поля |
|---|---|
| `StatusResult` | — |
| `OperationResult` | `automaton: Option<AutomatonData>` |
| `StateResult` | `state: Option<StateData>` |
| `TransitionResult` | `transition: Vec<TransitionData>` |
| `RunResult` | `traces: Vec<Trace>` |
| `MultiRunResult` | `traces: Vec<LineTest>` |
| `MealyRunResult` | `output: String`, `steps: Vec<MealyRunStep>` |
| `MealyMultiRunResult` | `results: Vec<MealyLineTest>` |
| `GenerateInputsResult` | `inputs: Vec<String>` |
| `LoadResult` | `isAutomaton: Option<bool>` (опускается при `None`), `data: Option<AutomatonData>` |

**Семантика статусов:**
- `200 OK` — успех / цепочка принята (обработана полностью);
- `400 BAD_REQUEST` — ошибка ввода, автомат не найден (в командах редактирования), некорректный автомат, ошибка файла;
- `401 UNAUTHORIZED` — цепочка принята/обработана **частично**;
- `402 PAYMENT_REQUIRED` — цепочка отклонена / не обработана ни на символ;
- `404 NOT_FOUND` — автомат не найден (в командах прогона/генерации).

Коды 401/402 используются как доменные статусы, а не по HTTP-смыслу — не менять без согласования с фронтендом.

### 6.3. Реестр `AutomatonStore`
`automata: Mutex<HashMap<i32, AutomatonData>>`, `next_id: Mutex<i32>` (с 1).
Методы: `new`, `create(name, initial_label)` (автомат типа `FA` с одним начальным
состоянием `id = 0` в (100, 200), `isFinal = Some(false)`), `insert(data)` (присваивает
новый id), `get(id) -> Option<AutomatonData>` (клон), `update(data)`, `remove(id)`, `list_ids()`.
Паттерн команд: `get` → изменить копию → `update`.

Идентификаторы: автоматы — последовательные из store; состояния и переходы — случайные
через `generate_id` (уникальны в пределах автомата).

### 6.4. Модели вычислений
- `FA { states: HashSet<i32>, alphabet: HashSet<char>, transitions: HashMap<(i32, char), HashSet<i32>>, initial_state: i32, final_states: HashSet<i32> }` — поля приватные, создание через `FABuilder` (`state(s)`, `symbol(s)`, `transition`, `epsilon`, `set_initial`, `set_final`, `final_states`, `build() -> Result<FA, String>`).
- `MM { states, input_alphabet, output_alphabet, transitions: HashMap<(i32, char), (i32, char)>, initial_state }` — поля приватные, создание через `MealyBuilder` (`state(s)`, `input(s)`, `output(s)`, `set_initial`, `transition(from, input, to, output)`, `build()`).
- `MealyTransition { from, input, to, output }`.
- Трейты `Automaton` (`accepts`, `states`, `initial_state`, `final_states`, `alphabet`, `is_accepting`) и `NondeterministicAutomaton` (`next_states`, `epsilon_closure`).

### 6.5. JFF (`core::jff::types`)
`JffAutomaton { states, transitions }`, `JffGrammar {}` (заглушка),
`enum JffParsed { Automaton(AutomatonType, JffAutomaton), Grammar(JffGrammar) }`.

## 7. API (Tauri-команды)

Фронтенд вызывает команды через `invoke("<имя>", { ... })`. Tauri преобразует
snake_case-аргументы Rust в **camelCase на стороне JS** (`automaton_id` → `automatonId`,
`is_initial` → `isInitial`, `new_from` → `newFrom` и т. д.). Параметр
`state: State<AutomatonStore>` внедряется Tauri и фронтендом не передаётся.

### 7.1. Общие (`core::api`)
| Команда | Аргументы | Ответ | Описание |
|---|---|---|---|
| `load_file` | `path: String` | `LoadResult` | Читает `.jff`, регистрирует объект в store. Для не-автоматов — заглушка (`isAutomaton = false`) |
| `save_file` | `automaton_id: i32`, `path: String` | `StatusResult` | Сохраняет автомат в `.jff` по его типу |

### 7.2. Конечные автоматы (`fa::api`)
| Команда | Аргументы | Ответ | Описание |
|---|---|---|---|
| `fa_create_new` | `name: Option<String>` | `OperationResult` | Новый КА с начальным `q0` (имя по умолчанию «Автомат») |
| `fa_add_state` | `automaton_id`, `label?`, `x?`, `y?`, `is_initial?`, `is_final?` | `StateResult` | Добавляет состояние (метка по умолчанию `q<id>`) |
| `fa_update_state` | `automaton_id`, `state_id`, `label?`, `x?`, `y?`, `is_initial?`, `is_final?` | `StateResult` | Частичное обновление |
| `fa_remove_state` | `automaton_id`, `state_id` | `StatusResult` | Удаляет состояние и все связанные переходы |
| `fa_add_transition` | `automaton_id`, `from`, `to`, `symbols: Vec<char>` | `TransitionResult` | По переходу на каждый символ; отклоняются только точные дубли `(from, to, symbol)`; `$` допустим |
| `fa_update_transition` | `automaton_id`, `transition_id`, `new_from?`, `new_to?`, `new_symbol?` | `TransitionResult` | Частичное обновление |
| `fa_remove_transition` | `automaton_id`, `transition_id` | `StatusResult` | Удаление перехода |
| `fa_run_str` | `automaton_id`, `input: String` | `RunResult` | Пошаговое моделирование (`run_partial`); трассы с `isFinal` первыми; 200/401/402 |
| `fa_multi_run_str` | `automaton_id`, `inputs: Vec<String>` | `MultiRunResult` | Пакетный прогон без трасс |
| `fa_generate_inputs` | `automaton_id` | `GenerateInputsResult` | Тестовые цепочки (`generate_test_inputs(50, 15)`) |
| `fa_remove_automaton` | `automaton_id` | `StatusResult` | Удаление автомата из store |

### 7.3. Машины Мили (`mealy::api`)
| Команда | Аргументы | Ответ | Описание |
|---|---|---|---|
| `mealy_create_new` | `name: Option<String>` | `OperationResult` | Новая машина (имя «Мили»), `automatonType = MEALY`, `isFinal = None` |
| `mealy_add_state` | как `fa_add_state` | `StateResult` | Делегирует в `fa_add_state`, `is_final` игнорируется |
| `mealy_update_state` | как `fa_update_state` | `StateResult` | Делегирует в `fa_update_state`, `is_final` игнорируется |
| `mealy_remove_state` | `automaton_id`, `state_id` | `StatusResult` | Делегирует в `fa_remove_state` |
| `mealy_add_transition` | `automaton_id`, `from`, `to`, `input: char`, `output: char` | `TransitionResult` | Запрещены eps и дубли `(from, input)` |
| `mealy_update_transition` | `automaton_id`, `transition_id`, `input?`, `output?` | `TransitionResult` | `from`/`to` не меняются; сохраняется детерминизм |
| `mealy_remove_transition` | `automaton_id`, `transition_id` | `StatusResult` | Удаление перехода |
| `mealy_run_str` | `automaton_id`, `input: String` | `MealyRunResult` | Прогон с выходной строкой; 200/401/402 |
| `mealy_multi_run` | `automaton_id`, `inputs: Vec<String>` | `MealyMultiRunResult` | Пакетный прогон; частичность — через `isComplete`, статус 200/400 |
| `mealy_remove_automaton` | `automaton_id` | `StatusResult` | Удаление автомата |

## 8. Формат JFLAP (`.jff`)
- Корень `<structure>`, тип в `<type>` (`fa`, `mealy`); далее `<automaton>` с `<state id name>`
  (`<x>`, `<y>`, `<initial/>`, `<final/>`) и `<transition>` (`<from>`, `<to>`, `<read>`, для Мили `<transout>`).
- FA: пустой/отсутствующий `<read>` ⇔ eps (`$`).
- Mealy: `<final/>` не пишется.
- Для новых типов: добавить ветку в `core::jff::parse_jff` и в `core::api::save_automaton`, создать модуль `<type>/jff`.

## 9. Соглашения и инварианты
- `EPSILON = '$'` — никогда не входит во входной алфавит; у Мили запрещён.
- Сообщения пользователю (`message`) — на русском; комментарии — на русском в шапке функции (см. раздел 10, п. 6).
- Команды не паникуют на пользовательском вводе: ошибки возвращаются через `status`/`message`.
- Валидация выполняется до мутации данных.
- Чистую логику выносить в `pub(crate)`-функции без `State`, чтобы тестировать без Tauri (пример — `mealy::api`).
- Тесты живут в `tests.rs` рядом с модулем (`#[cfg(test)] mod tests;`). Любое изменение логики сопровождается тестами; перед завершением — `cargo test`, `cargo clippy`, `cargo fmt`.
- Добавление нового типа автомата: вариант в `AutomatonType` → модуль `src/<type>/{mod,types,api,jff,tests}.rs` → `mod` в `lib.rs` → регистрация команд → ветки в `parse_jff`/`save_automaton` → обновление фронтенда (только упомянуть!).
- **Известная ловушка:** `fa::api::data_to_fa` вызывает `state.isFinal.unwrap()`; состояние КА с `isFinal = None` (например, `fa_add_state` без `is_final`) приведёт к панике при прогоне.

## 10. Правила для ИИ-агентов

1. Изменять только файлы в `src-tauri/`. Необходимые изменения вне этой директории — **только описывать** (путь, суть изменения), не выполняя их.
2. Не ломать контракт с фронтендом (имена команд, аргументов, полей DTO, коды статусов) без явного указания; при изменении контракта — перечислить, что нужно поправить в `../src/api/*.ts`.
3. Новые команды регистрировать в `lib.rs` (`generate_handler!`).
4. После изменений прогонять `cargo test` и `cargo clippy`.
5. **Обозначение eps.** Во всех местах в коде (комментарии, doc-комментарии, сообщения
   `message`, имена новых переменных, функций и тестов) вместо латинского `epsilon`
   и символа `ε` писать просто `eps`. Пример: `// eps-замыкание`, `eps_closure`,
   `"Машина Мили не поддерживает eps-переходы"`.
6. **Комментарии — на русском и в шапке функции.** Все комментарии пишутся на русском
   языке в виде doc-комментария `///` непосредственно над функцией (методом, командой):
   что делает функция, аргументы, возвращаемое значение, ошибки/статусы. Не добавлять
   комментарии внутри тела функции; модули описываются `//!` в начале файла, тоже на русском.
7. **camelCase в контракте с фронтендом.** Все поля структур, входящих в контракт
   с фронтендом (DTO и ответы команд из `core::types`: всё, что сериализуется или
   десериализуется serde и уходит в `invoke` или приходит из него), именуются в
   стиле camelCase: `isInitial`, `automatonType`, `correctSymbols`. Как и в текущем
   коде, поле называется в camelCase прямо в Rust, а над структурой ставится
   `#[allow(non_snake_case)]`. Внутренние структуры (`FA`, `MM`, билдеры) остаются в
   snake_case. Аргументы Tauri-команд пишутся в snake_case: Tauri сам переводит их в
   camelCase на стороне JS.
8. **Актуализация этого файла обязательна.** Если ИИ-агент что-либо изменяет в проекте
   (добавляет/удаляет/переименовывает модули, файлы, команды API, структуры, поля,
   статусы, зависимости, соглашения), он **обязан в том же изменении обновить
   `AGENTS.md`**, чтобы описание соответствовало коду. Изменение без обновления
   `AGENTS.md` считается незавершённым.
