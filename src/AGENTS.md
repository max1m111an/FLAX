# AGENTS.md — FLAX (frontend, `src`)

Руководство для ИИ-агентов, работающих с клиентской частью FLAX.

## 1. О проекте

**FLAX (Formal Languages and Automata eXplorer)** — учебный пакет для моделирования
конечных автоматов и формальных языков (аналог JFLAP). Десктопное приложение на
**Tauri 2**: фронтенд на **React 19 + TypeScript + Vite** (этот каталог), бэкенд на
**Rust** (каталог `../src-tauri`, у него свой `AGENTS.md`).

Фронтенд отвечает за:
- главный экран (создание моделей, открытие файлов, история недавних файлов);
- систему вкладок (несколько автоматов одновременно, признак несохранённых изменений);
- холст редактора: состояния, переходы, контекстное меню, перетаскивание;
- тестирование цепочек: одиночный пошаговый прогон и пакетный прогон;
- открытие и сохранение `.jff` через системные диалоги;
- настройки (тема оформления).

Вся вычислительная логика и хранение автоматов находятся на бэкенде. Фронтенд вызывает
его через `invoke` из `@tauri-apps/api/core`.

Поддерживаемые типы моделей: `FA` (конечный автомат), `MEALY` (машина Мили).
`AT` (машина Тьюринга), `MOORE`, `GRAMMAR` есть в каталоге моделей (`data/Models.ts`),
но сервисов для них пока нет.

## 2. Границы рабочей области (ОБЯЗАТЕЛЬНО)

- **Рабочая директория агента — `src/`.** Изменять можно только файлы внутри неё.
- Если задача затрагивает файлы **вне** `src/` (бэкенд `../src-tauri`, `../package.json`,
  `../vite.config.ts`, `../tsconfig*.json`, `../eslint.config.js`, `../index.html`,
  `../public/`, `../Makefile`, `../.github/` и т. д.), агент **обязан явно упомянуть**,
  какие файлы и как нужно изменить, **но не изменять их**.
- Типичный случай: новая или изменённая Tauri-команда. Агент меняет только
  `src/api/*` и `src/types/api/*`, а нужные правки в Rust (`../src-tauri/src/...`)
  описывает словами.
- Новую npm-зависимость тоже не ставить самостоятельно, а указать, что её нужно
  добавить в `../package.json`.

## 3. Технологии и сборка

| Что | Значение |
|---|---|
| UI | React 19, react-router-dom 7 (`createHashRouter`), `@radix-ui/react-dropdown-menu`, `clsx` |
| Язык | TypeScript ~5.8, `strict`, `noUnusedLocals`, `noUnusedParameters` |
| Сборка | Vite 7, dev-сервер на порту 1420; плагины `@vitejs/plugin-react`, `vite-plugin-svgr` |
| Стили | SCSS, CSS-модули (`*.module.scss`); `consts.scss` подключается автоматически в каждый SCSS-файл |
| Tauri | `@tauri-apps/api`, `@tauri-apps/plugin-dialog` (`open`, `save`), `@tauri-apps/plugin-fs`, `@tauri-apps/plugin-opener` |
| Алиас | `@/` → `src/` (импорты пишутся как `@/types/Automaton.ts`) |
| SVG | импорт как компонент: `import Icon from "@/assets/svg/Name.svg?react"` |

Команды (из корня репозитория):
```bash
npm run dev          # только Vite
npm run tauri dev    # приложение целиком
npm run build        # tsc + vite build
npm run lint         # eslint src
npm run lint:fix     # eslint с автоисправлением
```

Стиль кода задан в `../eslint.config.js`: отступ 4 пробела, двойные кавычки, точка с
запятой обязательна, висячая запятая в многострочных конструкциях, пробелы внутри
`{ }` и `[ ]`, пробелы внутри фигурных скобок в JSX (`{ value }`), скобки у стрелочных
функций всегда. `any` разрешён; неиспользуемые переменные допускаются только с префиксом `_`.

## 4. Структура каталога

```
src/
├── AGENTS.md                # это руководство
├── refact.md                # аудит и план рефакторинга
├── Main.tsx                 # точка входа: ThemeProvider + RouterProvider
├── vite-env.d.ts
├── api/                     # тонкие обёртки над invoke (по одной на группу команд)
│   ├── faAPI.ts             # faApi    → команды fa_*
│   ├── MealyAPI.ts          # mealyApi → команды mealy_*
│   └── jffAPI.ts            # jffAPI   → load_file / save_file
├── services/                # бизнес-логика поверх api
│   ├── IAutomatonService.ts # общий интерфейс сервиса автомата
│   ├── ModelServiceRegistry.ts # выбор сервиса по типу автомата
│   ├── faService.ts
│   ├── mealyService.ts
│   ├── jffService.ts
│   └── dirtyState.ts        # глобальный стор «несохранённых» автоматов
├── types/
│   ├── Automaton.ts         # модели автомата, состояния, перехода; model (карточка)
│   ├── Payloads.ts          # AutomatonType, payload-ы редактирования
│   ├── Tabs.ts              # вкладки
│   ├── Testing.ts           # шаги прогона, трассы, результаты пакетного прогона
│   └── api/                 # типы запросов/ответов Tauri-команд
│       ├── common.ts        # ApiResponse<T>
│       ├── fa.ts
│       ├── mealy.ts
│       └── jff.ts
├── context/                 # React-контексты
│   ├── TabsContext.tsx
│   ├── RecentFilesContext.tsx
│   ├── SnackbarContext.tsx
│   ├── ThemeContext.tsx
│   └── ContextMenuContext.tsx (+ .module.scss)
├── hooks/
│   ├── useStateAction.tsx   # useStateActions — операции над состояниями
│   ├── useTransitionAction.tsx # операции над переходами
│   └── useTesting.tsx       # пошаговый прогон
├── configs/
│   ├── RoutesConfig.tsx     # роутер
│   ├── RoutesConst.ts       # ROUTES
│   ├── CanvasConst.ts       # размеры узлов на холсте
│   └── CommonConst.ts       # общие константы (EPS)
├── data/Models.ts           # каталог моделей для главного экрана
├── utils/
│   ├── calculatePoints.ts   # геометрия рёбер, петли (buildSelfLoop)
│   └── tabFactory.ts        # buildTab
├── layouts/MainLayout.tsx   # провайдеры + TabsWidget + <Outlet/>
├── scenes/                  # страницы роутера
│   ├── MainScene.tsx, ModelScene.tsx, SettingsScene.tsx, DebugScene.tsx
├── widgets/                 # крупные блоки экранов
│   ├── Main/        MainControlWidget, MainModelWidget, MainHistoryWidget
│   ├── Model/       ModelCanvasWidget, ModelControlWidget
│   │   └── ModelTesting/ ModelTestWidget, SoloTesting, MultiTesting, multiTestingConfig
│   └── Navigation/  TabsWidget
├── components/              # переиспользуемые компоненты
│   ├── State/, Edge/ (Transition), TraceCard/, Snackbar/, ConfirmCloseModal/
│   └── ui/ Button, IconButton, Switch, Textfield (+TextArea), Typography
└── assets/
    ├── scss/ _themes.scss, consts.scss, index.scss, main.scss
    └── svg/  иконки
```

## 5. Архитектура и поток данных

```
Компонент / виджет
   → хук (useStateActions, useTransitionAction, useTesting) или TabsContext
      → сервис (modelServiceRegistry.getService(type) → faService | mealyService; jffService)
         → api (faApi | mealyApi | jffAPI) → invoke("<команда>", { camelCase-аргументы })
            → Rust (src-tauri)
```

Правила слоёв:
- `invoke` вызывается **только** в `src/api/*`. Компоненты, хуки и контексты к api
  напрямую не обращаются, только через сервисы.
- Сервис отмечает автомат как изменённый (`markDirty`) после каждой мутации;
  `jffService.saveJff` снимает отметку.
- Источник правды об автомате — бэкенд. Фронтенд хранит копию в `tab.automaton` и
  обновляет её по ответу команды через `updateTab`.
- Тип конкретного автомата выбирается через `modelServiceRegistry`, а не через
  `if (type === ...)` в компонентах.

## 6. Модули

### 6.1. `api/`
Объекты `faApi`, `mealyApi`, `jffAPI`. Каждый метод вызывает одну команду, логирует
ошибку в консоль и пробрасывает её дальше.

| Объект | Метод | Команда |
|---|---|---|
| `faApi` | `createAutomaton(name)` | `fa_create_new` |
| | `deleteAutomaton(automatonId)` | `fa_remove_automaton` |
| | `addState(params)` | `fa_add_state` |
| | `updateState(params)` | `fa_update_state` |
| | `removeState(params)` | `fa_remove_state` |
| | `addTransition(params)` | `fa_add_transition` |
| | `updateTransit(params)` | `fa_update_transition` |
| | `removeTransit(params)` | `fa_remove_transition` |
| | `runStr(params)` | `fa_run_str` |
| | `multiRunStr(params)` | `fa_multi_run_str` |
| | `generateInputs(params)` | `fa_generate_inputs` |
| `mealyApi` | `createAutomaton(name)` | `mealy_create_new` |
| | `deleteAutomaton(automatonId)` | `mealy_remove_automaton` |
| | `addState` / `updateState` / `removeState` | `mealy_add_state` / `mealy_update_state` / `mealy_remove_state` |
| | `addTransition` / `updateTransitMealy` / `removeTransit` | `mealy_add_transition` / `mealy_update_transition` / `mealy_remove_transition` |
| | `runStr(params)` | `mealy_run_str` |
| | `multiRunStr(params)` | `mealy_multi_run` |
| `jffAPI` | `loadJFF({ path })` | `load_file` |
| | `saveJFF({ automatonId, path })` | `save_file` |

### 6.2. `services/`
- **`IAutomatonService<TAutomaton, TState, TTransition, TStatePayload, TRunResponse, TMultiRunResponse>`**:
  `createAutomaton(name)`, `deleteAutomaton(id)`, `addState(automatonId, payload)`,
  `updateState(automatonId, stateId, payload)`, `removeState(automatonId, stateId)`,
  `addTransition(automatonId, from, to, symbolStr)`, `removeTransition(automatonId, transitionId)`,
  `runStr(automatonId, input)`, `runMultipleStr(automatonId, inputs)`, `generateTestInputs(automatonId)`.
- **`faService`**. `addTransition` разбивает `symbolStr` по запятым; пустая строка
  означает eps-переход `EPS`. `addState` требует все поля payload.
- **`mealyService`**. `addTransition` разбирает `symbolStr` на пары «вход/выход»:
  фрагменты через пробел, внутри фрагмента разделители `→ - , ;` или два символа подряд
  (`ab` = вход `a`, выход `b`); для каждой пары отдельный вызов команды. `runStr`
  оборачивает ответ Мили в формат `traces` (одна трасса, `isFinal = status === 200`).
- **`modelServiceRegistry.getService(type)`** возвращает сервис по `AutomatonType`
  (регистр не важен) и бросает ошибку, если тип не поддерживается. Новый тип
  регистрируется в объекте `services` и в `isSupportedType`.
- **`jffService`**: `loadJff(params)`, `saveJff(params)`. Бросают ошибку при
  `status !== 200`.
- **`dirtyState`**: `markDirty(automatonId, modified)`, `isDirty(id)`,
  `subscribeDirty(listener)`, `getDirtySnapshot()` (для `useSyncExternalStore`),
  `stripDirtyStar(title)`.

### 6.3. `context/`
| Контекст | Хук | Содержимое |
|---|---|---|
| `TabsContext` | `useTabs()`, `useCurrentTab()` | `tabs`, `addTab(model, type?)`, `addUtilTab(type, icon)`, `removeTab(tab)` (с подтверждением при несохранённых изменениях), `updateTab(tab)`, `loadTab(path)`, `saveTab(tab, isSaveAs?)`. `useCurrentTab` берёт вкладку по `:id` из URL |
| `RecentFilesContext` | `useRecentFiles()` | `files: RecentFile[]`, `addFile(path, icon)`, `removeFile(path)`, `clearFiles()`; `localStorage["recent-files"]`, не больше 10 записей |
| `SnackbarContext` | `useSnackbar()` | `showSnackbar(type: "warning" \| "success" \| "error", message)` |
| `ThemeContext` | `useTheme()` | `theme: "Темная" \| "Светлая"`, `setTheme`; `localStorage["app-theme"]`, атрибут `data-theme` |
| `ContextMenuContext` | `useContextMenu()` | `setMenuState(...)`; компоненты `ContextMenuItem`, `ContextMenuInput` |

Порядок провайдеров (`layouts/MainLayout.tsx`): `ContextMenu → Snackbar → RecentFiles → Tabs`.
`ThemeProvider` подключается выше, в `Main.tsx`.

### 6.4. `hooks/`
- **`useStateActions()`**: `addState`, `removeState`, `handleNameChange`,
  `handleInitialChange`, `handleFinalChange`, `handleXChange`, `handleYChange`,
  `saveStatePosition`, `moveStateLocal`. Если активна вкладка `SETTINGS`, возвращает
  пустые заглушки.
- **`useTransitionAction()`**: `addTransition`, `removeTransition`.
- **`useTesting()`**: пошаговый прогон для `SoloTesting`. Возвращает `testLine`,
  `setTestLine`, `isPlay`, `currentIndex`, `symbols`, `traces`, `currentTab`,
  `handlePlay`, `handleStep`, `handleStepBack`, `handleFastForward`, `handleReset`.

### 6.5. Экраны, виджеты, компоненты
- Роуты (`ROUTES`): `/` → `MainScene`, `/settings` → `SettingsScene`,
  `/models/:id` → `ModelScene` (`id` = id автомата и вкладки), `/debug` → `DebugScene`.
- `MainScene`: `MainControlWidget` (открыть файл, настройки), `MainModelWidget`
  (карточки из `data/Models.ts`), `MainHistoryWidget` (недавние файлы).
- `ModelScene`: `ModelControlWidget` (инструменты, сохранение), `ModelCanvasWidget`
  (холст), `ModelTestWidget` (тесты: `SoloTesting` / `MultiTesting`; колонки таблицы
  пакетного прогона задаются в `TABLE_CONFIGS` в `multiTestingConfig.tsx` по типу автомата).
- `TabsWidget`: панель вкладок, звёздочка у несохранённых.
- Компоненты холста: `State` (узел), `Transition` (ребро, `React.memo`), `TraceCard`
  (трасса прогона). Базовые UI-компоненты лежат в `components/ui/`.

## 7. Типы и контракт с бэкендом

### 7.1. Общий ответ
```ts
type ApiResponse<T = Record<string, never>> = { status: number; message: string } & T;
```
Коды `status` задаёт бэкенд:
- `200` — успех, цепочка принята или обработана полностью;
- `400` — ошибка ввода или некорректный автомат;
- `401` — цепочка обработана частично;
- `402` — цепочка отклонена;
- `404` — автомат не найден (в командах прогона и генерации).

### 7.2. Модели (`types/Automaton.ts`)
| Тип | Поля |
|---|---|
| `BaseStateModel` | `id`, `label`, `x`, `y`, `isInitial` |
| `StateFAModel` | `+ isFinal: boolean` |
| `StateMealyModel` | `= BaseStateModel` |
| `BaseTransitionModel` | `id`, `from`, `to` |
| `TransitionFAModel` | `+ symbol: string` (`"$"` = eps) |
| `TransitionMealyModel` | `+ symbol: string`, `output: string` |
| `BaseAutomatonModel<TState, TTransition>` | `id`, `automatonType: AutomatonType`, `name`, `states`, `transitions` |
| `FAAutomatonModel`, `MealyAutomatonModel`, `AutomatonModel` | конкретизации и их объединение |
| `model` | карточка главного экрана: `title`, `type`, `icon`, `description` |

`char` из Rust на фронтенде представлен строкой из одного символа.

### 7.3. Тестирование (`types/Testing.ts`)
`RunStepFA { from, symbol, to }`, `RunStepMealy { from, symbol, to, output }`,
`Trace<T> { steps: T[], isFinal }`, `LineTestFA { line, isFinal, correctSymbols }`,
`LineTestMealy { line, output, isComplete }`, объединения `RunStep`, `LineTest`.

### 7.4. Payload-ы и вкладки
- `types/Payloads.ts`: `AutomatonType = "FA" | "SETTINGS" | "MEALY" | "AT" | "GRAMMAR" | "MOORE"`;
  `FAStatePayload` / `MealyStatePayload` и `FATransitionPayload` / `MealyTransitionPayload`
  с дискриминатором `type`.
- `types/Tabs.ts`: `BaseTab { id, title, type, isSaved, savedPath, icon }`;
  `FATab` (используется для всех автоматов) добавляет `automaton`,
  `activeControl: "move" | "edge" | "node" | "trashcan"`, `activePanel`,
  `selectedState`, `selectedTransition` (`TraceHighlight[]`), `selectedNodeId`,
  `testMode`, `testInput`, `pendingTestLine`, `pendingTraces`; `SettingsTab`;
  `AppTab = FATab | SettingsTab`. Вкладка создаётся через `utils/tabFactory.buildTab`.

### 7.5. Запросы (`types/api/*.ts`)
Аргументы `invoke` передаются в **camelCase**: Tauri сам сопоставляет их с
snake_case-параметрами Rust (`automatonId` → `automaton_id`, `newSymbol` → `new_symbol`).
Имена типов: `<Действие><Сущность><FA|Mealy|JFF>Request/Response`, например
`AddTransitFARequest`, `RunStrMealyResponse`, `LoadJFFRequest`.

## 8. Соглашения
- Импорты через алиас `@/`, с расширением файла (`.ts` / `.tsx`).
- Константа eps-символа — `EPS` из `configs/CommonConst.ts`; литерал `"$"` в коде
  не дублируется (значение остаётся `"$"` в контракте с бэкендом).
- Компоненты лежат в папке с одноимённым `.tsx` и `.module.scss`; стили только через
  CSS-модули и переменные темы из `_themes.scss` (`var(--bg-base)` и т. п.), без
  жёстко заданных цветов.
- Тексты интерфейса и сообщения об ошибках — на русском.
- Ошибки команд показываются пользователю через `showSnackbar`; в консоль они
  логируются в `api/`.
- После изменений нужно прогнать `npm run lint` и `npm run build` (проверка типов).

**Известные несоответствия (на момент написания):**
- `mealyService` объявлен с `// @ts-expect-error`: в нём нет `generateTestInputs`
  (на бэкенде нет такой команды для Мили).
- `updateTransitMealyRequest` содержит `newFrom` и `newTo`, но `mealy_update_transition`
  на бэкенде их не принимает: меняются только `input` и `output`.
- Тексты логов в `jffAPI.ts` (`fa_load`, `fa_save`) и в `mealyApi.addState`
  (`fa_add_state`) не совпадают с именами вызываемых команд.
- `faService.addState` на любое отсутствующее поле бросает одну и ту же ошибку «Ожидался FA».

## 9. Правила для ИИ-агентов

1. Изменять только файлы в `src/`. Необходимые изменения вне этой директории
   (в первую очередь в `../src-tauri`) **только описывать**: путь и суть изменения,
   не выполняя их.
2. Не ломать контракт с бэкендом: имена команд, аргументов, полей, коды статусов.
   Если контракт меняется, обновить `src/api/*` и `src/types/api/*` и перечислить,
   что нужно поправить в Rust.
3. `invoke` использовать только в `src/api/`; новые типы автоматов подключать через
   сервис, реализующий `IAutomatonService`, и `modelServiceRegistry`.
4. После изменений прогонять `npm run lint` и `npm run build`.
5. **Обозначение eps.** Во всех местах в коде (комментарии, JSDoc, тексты интерфейса и
   сообщений, имена новых переменных, функций, типов и CSS-классов) вместо латинского
   `epsilon` и символа `ε` писать просто `eps`. Пример: `// eps-переход`,
   `isEpsTransition`, `"Пустой ввод создаёт eps-переход"`. Значение eps-символа в
   контракте с бэкендом остаётся `"$"`.
6. **Комментарии — на русском и в шапке функции.** Все комментарии пишутся на русском
   языке в виде JSDoc-комментария `/** ... */` непосредственно над функцией, компонентом,
   хуком или методом: что делает, параметры/props, возвращаемое значение, ошибки.
   Не добавлять комментарии внутри тела функции или в JSX.
7. **camelCase в контракте с бэкендом.** Все поля типов, входящих в контракт с бэкендом
   (`types/api/*`, `types/Automaton.ts`, `types/Testing.ts`: всё, что передаётся в
   `invoke` или приходит из него), именуются в стиле camelCase (`automatonId`,
   `isInitial`, `correctSymbols`) и должны совпадать с именами полей DTO и аргументов
   команд на бэкенде.
8. **Актуализация этого файла обязательна.** Если ИИ-агент что-либо изменяет в проекте
   (добавляет, удаляет или переименовывает модули, файлы, компоненты, хуки, контексты,
   вызовы API, типы, поля, роуты, зависимости, соглашения), он **обязан в том же
   изменении обновить `AGENTS.md`**, чтобы описание соответствовало коду. Изменение
   без обновления `AGENTS.md` считается незавершённым.
