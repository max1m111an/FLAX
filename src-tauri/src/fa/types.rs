use std::collections::{HashMap, HashSet, VecDeque};

use crate::core::automaton::{Automaton, NondeterministicAutomaton};
use crate::core::types::{RunStep, Trace};

pub use crate::core::types::EPSILON;

/// Жёсткое ограничение числа параллельных потоков чтения в `run_partial`.
/// Достаточно велико, чтобы обойти все ветви реальных (учебных) КА, и при этом
/// ограничивает экспоненциальный рост в худшем случае.
const MAX_THREADS: usize = 1_000_000;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FA {
    states: HashSet<i32>,
    alphabet: HashSet<char>,
    transitions: HashMap<(i32, char), HashSet<i32>>,
    initial_state: i32,
    final_states: HashSet<i32>,
}

#[allow(dead_code)]
impl FA {
    pub fn new(
        states: HashSet<i32>,
        alphabet: HashSet<char>,
        transitions: HashMap<(i32, char), HashSet<i32>>,
        initial_state: i32,
        final_states: HashSet<i32>,
    ) -> Result<Self, String> {
        if !states.contains(&initial_state) {
            return Err(format!(
                "Начальное состояние '{}' не найдено в множестве состояний",
                initial_state
            ));
        }
        for state in &final_states {
            if !states.contains(state) {
                return Err(format!(
                    "Допускающее состояние '{}' не найдено в множестве состояний",
                    state
                ));
            }
        }
        for ((from, symbol), to_states) in &transitions {
            if !states.contains(from) {
                return Err(format!(
                    "Исходное состояние '{}' перехода не найдено в множестве состояний",
                    from
                ));
            }
            if *symbol != EPSILON && !alphabet.contains(symbol) {
                return Err(format!("Символ '{}' не принадлежит алфавиту", symbol));
            }
            for to in to_states {
                if !states.contains(to) {
                    return Err(format!(
                        "Целевое состояние '{}' перехода не найдено в множестве состояний",
                        to
                    ));
                }
            }
        }
        Ok(FA {
            states,
            alphabet,
            transitions,
            initial_state,
            final_states,
        })
    }

    pub fn builder() -> FABuilder {
        FABuilder::new()
    }

    pub fn get_transitions(&self) -> &HashMap<(i32, char), HashSet<i32>> {
        &self.transitions
    }

    pub fn reachable_states(&self) -> HashSet<i32> {
        let mut reachable: HashSet<i32> = HashSet::new();
        let mut stack: Vec<i32> = vec![self.initial_state];
        while let Some(current) = stack.pop() {
            if reachable.contains(&current) {
                continue;
            }
            reachable.insert(current);
            for symbol in self
                .alphabet
                .iter()
                .copied()
                .chain(std::iter::once(EPSILON))
            {
                if let Some(next_states) = self.transitions.get(&(current, symbol)) {
                    for next in next_states {
                        if !reachable.contains(next) {
                            stack.push(*next);
                        }
                    }
                }
            }
        }
        reachable
    }

    pub fn is_empty(&self) -> bool {
        if self.final_states.is_empty() {
            return true;
        }
        let reachable = self.reachable_states();
        !reachable.iter().any(|s| self.final_states.contains(s))
    }

    /// `true`, если автомат детерминирован: нет eps-переходов и не более одного
    /// целевого состояния на пару `(state, symbol)`. Детерминизм — свойство
    /// конечного автомата (проверка/семантика), а не отдельный тип: одна и та же
    /// структура может прогоняться любым способом.
    pub fn is_deterministic(&self) -> bool {
        !self.alphabet.contains(&EPSILON)
            && self.transitions.iter().all(|((_, symbol), targets)| {
                *symbol != EPSILON && targets.len() == 1
            })
    }

    pub fn run(&self, input: &[char]) -> Option<Vec<RunStep>> {
        if input.iter().any(|s| !self.alphabet.contains(s)) {
            return None;
        }

        let mut steps: Vec<RunStep> = Vec::new();
        let mut visited: HashSet<(i32, usize)> = HashSet::new();

        fn explore(
            fa: &FA,
            state: i32,
            pos: usize,
            input: &[char],
            steps: &mut Vec<RunStep>,
            visited: &mut HashSet<(i32, usize)>,
        ) -> bool {
            if !visited.insert((state, pos)) {
                return false;
            }

            if pos == input.len() {
                if fa.final_states.contains(&state) {
                    return true;
                }
                if let Some(next_states) = fa.transitions.get(&(state, EPSILON)) {
                    for &next in next_states {
                        steps.push(RunStep {
                            from: state,
                            symbol: EPSILON,
                            to: next,
                        });
                        if explore(fa, next, pos, input, steps, visited) {
                            return true;
                        }
                        steps.pop();
                    }
                }
                return false;
            }

            if let Some(next_states) = fa.transitions.get(&(state, EPSILON)) {
                for &next in next_states {
                    steps.push(RunStep {
                        from: state,
                        symbol: EPSILON,
                        to: next,
                    });
                    if explore(fa, next, pos, input, steps, visited) {
                        return true;
                    }
                    steps.pop();
                }
            }

            let symbol = input[pos];
            if let Some(next_states) = fa.transitions.get(&(state, symbol)) {
                for &next in next_states {
                    steps.push(RunStep {
                        from: state,
                        symbol: symbol,
                        to: next,
                    });
                    if explore(fa, next, pos + 1, input, steps, visited) {
                        return true;
                    }
                    steps.pop();
                }
            }

            false
        }

        if explore(self, self.initial_state, 0, input, &mut steps, &mut visited) {
            Some(steps)
        } else {
            None
        }
    }

    /// Параллельный прогон по потокам КА в стиле JFLAP (JFLAP «Step with Closure»).
    /// Возвращает (traces, accepted). Каждый элемент внешнего вектора — одно
    /// параллельное чтение (поток): история его шагов и признак завершения в
    /// финальном состоянии. Каждый отдельный путь хранится как собственная трасса,
    /// поэтому ветви, попавшие в одно состояние (или разветвившиеся по одному
    /// символу), дают отдельные трассы.
    ///
    /// eps-замыкание вычисляется перед первым символом, после каждого символа и
    /// после последнего символа; каждый `$`-переход, пройденный при замыкании,
    /// записывается в историю ветви как `RunStep` с символом "$". Замыкание
    /// расширяет множество состояний без ветвления; недетерминизм ветвится только
    /// по символьным переходам.
    ///
    /// Сообщается о каждой ветви, включая досрочно завершившиеся: поток, который
    /// не может прочитать текущий символ, финализируется (сохраняется с частичной
    /// историей и `isFinal = false`), а не отбрасывается. На `accepted` влияют
    /// только потоки, прочитавшие весь вход, замкнутые и оказавшиеся в финальном
    /// состоянии.
    ///
    /// Семантика:
    /// - Если в автомате нет финальных состояний, любая строка отклоняется.
    /// - Каждый символ читается всеми потоками, которые могут его прочитать;
    ///   символ вне алфавита останавливает чтение прямо перед ним, и строка
    ///   отклоняется.
    /// - Число потоков ограничено, чтобы сдержать экспоненциальный рост в худшем
    ///   случае.
    pub fn run_partial(&self, input: &[char]) -> (Vec<Trace>, bool) {
        // Финализация потока в конце входа: применяем итоговое eps-замыкание
        // (записывая его шаги `$`) и ставим isFinal, если хотя бы одно состояние финальное.
        let finalize_end = |fa: &Self, thread: (Vec<RunStep>, i32)| -> Trace {
            let (closure_set, closure_steps) = fa.epsilon_closure_with_steps(thread.1);
            let mut steps = thread.0;
            steps.extend(closure_steps);
            let is_final = closure_set.iter().any(|s| fa.final_states.contains(s));
            Trace {
                steps,
                isFinal: is_final,
            }
        };

        if self.final_states.is_empty() {
            return (Vec::new(), false);
        }

        let mut result: Vec<Trace> = Vec::new();
        let mut threads: Vec<(Vec<RunStep>, i32)> = vec![(Vec::new(), self.initial_state)];

        for &symbol in input {
            // Символ вне алфавита: останавливаем чтение прямо перед ним и отклоняем,
            // финализируя живые на данный момент потоки (частичный прогресс).
            if !self.alphabet.contains(&symbol) {
                result.extend(threads.into_iter().map(|t| finalize_end(self, t)));
                return (result, false);
            }

            let mut next_threads: Vec<(Vec<RunStep>, i32)> = Vec::new();
            'outer: for (history, state) in &threads {
                // 1) eps-замыкание состояния этого потока ПЕРЕД символом; каждый
                //    использованный `$`-переход записывается в историю ветви.
                let (closure_set, closure_steps) = self.epsilon_closure_with_steps(*state);
                let mut hist = history.clone();
                hist.extend(closure_steps);

                // 2) Ветвимся по каждому символьному переходу из любого состояния замыкания;
                //    каждое целевое состояние порождает отдельный поток (со своей историей).
                let mut advanced = false;
                for from in &closure_set {
                    if let Some(targets) = self.transitions.get(&(*from, symbol)) {
                        advanced = true;
                        for &t in targets {
                            if next_threads.len() >= MAX_THREADS {
                                break 'outer;
                            }
                            let mut h = hist.clone();
                            h.push(RunStep {
                                from: *from,
                                symbol: symbol,
                                to: t,
                            });
                            next_threads.push((h, t));
                        }
                    }
                }

                // 3) Поток не может прочитать текущий символ: он прерывается и сохраняется
                //    с частичной историей и isFinal = false.
                if !advanced {
                    result.push(Trace {
                        steps: hist,
                        isFinal: false,
                    });
                }
            }

            // Ни один живой поток не смог прочитать этот символ: остаётся только
            // вернуть то, что уже финализировано (частичный прогресс).
            if next_threads.is_empty() {
                return (result, false);
            }

            if next_threads.len() >= MAX_THREADS {
                break;
            }

            threads = next_threads;
        }

        // Конец входа: финализируем каждый выживший поток (с итоговым замыканием).
        let mut accepted = false;
        for thread in threads {
            let trace = finalize_end(self, thread);
            if trace.isFinal {
                accepted = true;
            }
            result.push(trace);
        }
        (result, accepted)
    }

    fn epsilon_closure_owned(&self, state: i32) -> HashSet<i32> {
        let mut closure: HashSet<i32> = HashSet::new();
        let mut stack: Vec<i32> = vec![state];
        while let Some(current) = stack.pop() {
            if closure.contains(&current) {
                continue;
            }
            closure.insert(current);
            if let Some(next_states) = self.transitions.get(&(current, EPSILON)) {
                for next in next_states {
                    if !closure.contains(next) {
                        stack.push(*next);
                    }
                }
            }
        }
        closure
    }

    /// eps-замыкание `state` и упорядоченный список `$`-переходов, пройденных для
    /// достижения каждого состояния замыкания. По `$`-переходам ветвления нет (каждый
    /// записывается в потоке один раз), как в JFLAP Step with Closure.
    fn epsilon_closure_with_steps(&self, state: i32) -> (HashSet<i32>, Vec<RunStep>) {
        let mut closure: HashSet<i32> = HashSet::new();
        let mut steps: Vec<RunStep> = Vec::new();
        let mut stack: Vec<i32> = vec![state];
        while let Some(current) = stack.pop() {
            if closure.contains(&current) {
                continue;
            }
            closure.insert(current);
            if let Some(next_states) = self.transitions.get(&(current, EPSILON)) {
                for &next in next_states {
                    if !closure.contains(&next) {
                        steps.push(RunStep {
                            from: current,
                            symbol: EPSILON,
                            to: next,
                        });
                        stack.push(next);
                    }
                }
            }
        }
        (closure, steps)
    }

    fn next_states_owned(&self, state: i32, symbol: char) -> HashSet<i32> {
        self.transitions
            .get(&(state, symbol))
            .cloned()
            .unwrap_or_default()
    }

    fn next_states_with_epsilon_owned(&self, state: i32, symbol: char) -> HashSet<i32> {
        let direct = self.next_states_owned(state, symbol);
        let mut result: HashSet<i32> = HashSet::new();
        for s in direct {
            result.extend(self.epsilon_closure_owned(s));
        }
        result
    }

    /// Кратчайшая строка из символов алфавита, достигающая каждого достижимого
    /// состояния. eps-переходы проходятся «бесплатно» (не добавляют символов), поэтому
    /// уровень BFS равен числу прочитанных символов и даёт минимальные строки.
    /// Состояния дальше `max_len` символов от начала не раскрываются.
    fn shortest_paths(&self, max_len: usize) -> HashMap<i32, String> {
        let mut dist: HashMap<i32, String> = HashMap::new();
        let mut queue: VecDeque<(i32, String, usize)> = VecDeque::new();
        dist.insert(self.initial_state, String::new());
        queue.push_back((self.initial_state, String::new(), 0));

        while let Some((state, path, depth)) = queue.pop_front() {
            let closure = self.epsilon_closure_owned(state);
            for t in &closure {
                dist.entry(*t).or_insert_with(|| path.clone());
            }
            if depth >= max_len {
                continue;
            }
            let mut moves: Vec<(char, i32)> = Vec::new();
            for ch in self.alphabet.iter().copied().filter(|c| *c != EPSILON) {
                for c in &closure {
                    if let Some(targets) = self.transitions.get(&(*c, ch)) {
                        for &t in targets {
                            moves.push((ch, t));
                        }
                    }
                }
            }
            for (ch, t) in moves {
                if !dist.contains_key(&t) {
                    let mut next_path = path.clone();
                    next_path.push(ch);
                    let next_depth = depth + 1;
                    dist.insert(t, next_path.clone());
                    queue.push_back((t, next_path, next_depth));
                }
            }
        }
        dist
    }

    /// Символы, не входящие в алфавит (цифры, затем буква, затем `$`), для
    /// негативных тестов. Возвращает до двух различных символов.
    fn outside_alphabet_symbols(&self) -> Vec<char> {
        let mut out: Vec<char> = Vec::new();
        let preferred: Vec<char> = (b'0'..=b'9')
            .chain(b'a'..=b'z')
            .chain(b'A'..=b'Z')
            .map(|b| b as char)
            .collect();
        let mut candidates: Vec<char> = vec!['0', '4', EPSILON];
        candidates.extend(preferred);
        for c in candidates {
            if !self.alphabet.contains(&c) && !out.contains(&c) {
                out.push(c);
                if out.len() == 2 {
                    break;
                }
            }
        }
        out
    }

    /// Кратчайшая строка-цикл (>= 1 символа), выходящая из `start` и возвращающаяся в него.
    fn shortest_cycle_for(&self, start: i32, max_len: usize) -> Option<String> {
        let mut queue: VecDeque<(i32, String, usize)> = VecDeque::new();
        let mut visited: HashSet<i32> = HashSet::new();

        let closure = self.epsilon_closure_owned(start);
        for ch in self.alphabet.iter().copied().filter(|c| *c != EPSILON) {
            for c in &closure {
                if let Some(targets) = self.transitions.get(&(*c, ch)) {
                    for &t in targets {
                        if t == start {
                            return Some(ch.to_string());
                        }
                        if !visited.contains(&t) {
                            visited.insert(t);
                            queue.push_back((t, ch.to_string(), 1));
                        }
                    }
                }
            }
        }

        while let Some((state, path, depth)) = queue.pop_front() {
            if depth >= max_len {
                continue;
            }
            let cl = self.epsilon_closure_owned(state);
            for ch in self.alphabet.iter().copied().filter(|c| *c != EPSILON) {
                for c in &cl {
                    if let Some(targets) = self.transitions.get(&(*c, ch)) {
                        for &t in targets {
                            if t == start {
                                let mut cycle = path.clone();
                                cycle.push(ch);
                                return Some(cycle);
                            }
                            if !visited.contains(&t) {
                                visited.insert(t);
                                let mut next_path = path.clone();
                                next_path.push(ch);
                                queue.push_back((t, next_path, depth + 1));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Находит любой цикл в автомате: состояние, из которого непустая строка
    /// возвращает в него же. Возвращает состояние и строку цикла.
    fn find_cycle(&self, max_len: usize) -> Option<(i32, String)> {
        for &s in &self.states {
            if let Some(cycle) = self.shortest_cycle_for(s, max_len) {
                return Some((s, cycle));
            }
        }
        None
    }

    /// Строки, корректные до последнего символа, но затем упирающиеся в тупик
    /// (из замыкания достигнутого состояния нет перехода по символам алфавита).
    fn out_of_bounds_strings(&self, dist: &HashMap<i32, String>, result: &mut HashSet<String>) {
        let mut states: Vec<i32> = dist.keys().copied().collect();
        states.sort_by_key(|s| dist[s].len());
        states.truncate(8);
        for state in states {
            let closure = self.epsilon_closure_owned(state);
            for ch in self.alphabet.iter().copied().filter(|c| *c != EPSILON) {
                let has_transition = closure
                    .iter()
                    .any(|c| self.transitions.contains_key(&(*c, ch)));
                if !has_transition {
                    result.insert(format!("{}{}", dist[&state], ch));
                }
            }
        }
    }

    /// Генерирует упорядоченный набор тестовых строк без дубликатов, покрывающий
    /// широкий спектр сценариев КА: пустой вход, одиночные символы, кратчайшие пути
    /// до каждого состояния, срабатывание каждого перехода, негативные случаи
    /// (символы вне алфавита, тупики, строки, заканчивающиеся в нефинальных
    /// состояниях) и циклические/длинные строки. Результат отсортирован по длине и
    /// ограничен `cap` элементами.
    pub fn generate_test_inputs(&self, cap: usize, depth: usize) -> Vec<String> {
        let mut result: HashSet<String> = HashSet::new();

        result.insert(String::new());
        for ch in self.alphabet.iter().copied().filter(|c| *c != EPSILON) {
            result.insert(ch.to_string());
        }

        let dist = self.shortest_paths(depth);

        let mut paths: Vec<(&i32, &String)> = dist.iter().collect();
        paths.sort_by_key(|(_, p)| p.len());
        for &(_, path) in &paths {
            result.insert(path.clone());
        }

        for ((from, symbol), _targets) in &self.transitions {
            if let Some(path) = dist.get(from) {
                if *symbol == EPSILON {
                    result.insert(path.clone());
                } else {
                    let mut activated = path.clone();
                    activated.push(*symbol);
                    result.insert(activated);
                }
            }
        }

        self.out_of_bounds_strings(&dist, &mut result);

        let outside = self.outside_alphabet_symbols();
        let mut bases: Vec<String> = Vec::new();
        for &s in &self.final_states {
            if let Some(p) = dist.get(&s) {
                bases.push(p.clone());
            }
        }
        for ch in self.alphabet.iter().copied().filter(|c| *c != EPSILON) {
            bases.push(ch.to_string());
        }
        bases.sort();
        bases.dedup();
        bases.sort_by(|a, b| a.len().cmp(&b.len()));
        bases.truncate(4);

        for o in &outside {
            let outside_str = o.to_string();
            result.insert(outside_str.clone());
            for base in &bases {
                result.insert(format!("{}{}", outside_str, base));
                result.insert(format!("{}{}", base, outside_str));
                let mid = base.len() / 2;
                let (left, right) = base.split_at(mid);
                result.insert(format!("{}{}{}", left, outside_str, right));
            }
        }

        if let Some((entry, cycle)) = self.find_cycle(depth) {
            let prefix = dist.get(&entry).cloned().unwrap_or_default();
            result.insert(format!("{}{}", prefix, cycle));
            result.insert(format!("{}{}{}", prefix, cycle, cycle));
            result.insert(format!("{}{}{}{}", prefix, cycle, cycle, cycle));
        }

        let longest = dist
            .values()
            .max_by_key(|p| p.len())
            .cloned()
            .unwrap_or_default();
        let mut long = longest.clone();
        if let Some((entry, cycle)) = self.find_cycle(depth) {
            let prefix = dist.get(&entry).cloned().unwrap_or_default();
            let mut walk = prefix;
            while walk.len() < 20 {
                walk.push_str(&cycle);
                if cycle.is_empty() {
                    break;
                }
            }
            walk.truncate(20);
            if walk.len() > long.len() {
                long = walk;
            }
        }
        if long.len() >= 12 {
            result.insert(long);
        }

        let mut sorted: Vec<String> = result.into_iter().collect();
        sorted.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
        sorted.truncate(cap);
        sorted
    }
}

impl Automaton for FA {
    type State = i32;
    type Symbol = char;

    fn accepts(&self, input: &[char]) -> bool {
        let mut current_states: HashSet<i32> = self.epsilon_closure_owned(self.initial_state);
        for &symbol in input {
            if !self.alphabet.contains(&symbol) {
                return false;
            }
            let mut next_states: HashSet<i32> = HashSet::new();
            for state in current_states {
                next_states.extend(self.next_states_with_epsilon_owned(state, symbol));
            }
            current_states = next_states;
            if current_states.is_empty() {
                return false;
            }
        }
        current_states.iter().any(|s| self.final_states.contains(s))
    }

    fn states(&self) -> &HashSet<i32> {
        &self.states
    }

    fn initial_state(&self) -> &i32 {
        &self.initial_state
    }

    fn final_states(&self) -> &HashSet<i32> {
        &self.final_states
    }

    fn alphabet(&self) -> &HashSet<char> {
        &self.alphabet
    }
}

impl NondeterministicAutomaton for FA {
    fn epsilon_closure(&self, state: &i32) -> HashSet<&i32> {
        self.epsilon_closure_owned(*state)
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|s| self.states.get(&s).unwrap())
            .collect()
    }

    fn next_states(&self, state: &i32, symbol: &char) -> HashSet<&i32> {
        self.transitions
            .get(&(*state, *symbol))
            .map(|states| states.iter().collect())
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct FABuilder {
    states: HashSet<i32>,
    alphabet: HashSet<char>,
    transitions: HashMap<(i32, char), HashSet<i32>>,
    initial_state: Option<i32>,
    final_states: HashSet<i32>,
}

#[allow(dead_code)]
impl FABuilder {
    pub fn new() -> Self {
        FABuilder::default()
    }

    pub fn state(mut self, state: i32) -> Self {
        self.states.insert(state);
        self
    }

    pub fn states(mut self, states: &[i32]) -> Self {
        for &state in states {
            self.states.insert(state);
        }
        self
    }

    pub fn symbol(mut self, symbol: char) -> Self {
        self.alphabet.insert(symbol);
        self
    }

    pub fn symbols(mut self, symbols: &[char]) -> Self {
        for &symbol in symbols {
            self.alphabet.insert(symbol);
        }
        self
    }

    pub fn transition(mut self, from: i32, symbol: char, to: i32) -> Self {
        self.states.insert(from);
        self.states.insert(to);
        self.alphabet.insert(symbol);
        self.transitions
            .entry((from, symbol))
            .or_default()
            .insert(to);
        self
    }

    pub fn epsilon(mut self, from: i32, to: i32) -> Self {
        self.states.insert(from);
        self.states.insert(to);
        self.transitions
            .entry((from, EPSILON))
            .or_default()
            .insert(to);
        self
    }

    pub fn set_initial(mut self, state: i32) -> Self {
        self.states.insert(state);
        self.initial_state = Some(state);
        self
    }

    pub fn set_final(mut self, state: i32) -> Self {
        self.states.insert(state);
        self.final_states.insert(state);
        self
    }

    pub fn final_states(mut self, states: &[i32]) -> Self {
        for &state in states {
            self.states.insert(state);
            self.final_states.insert(state);
        }
        self
    }

    pub fn build(self) -> Result<FA, String> {
        let initial_state = self.initial_state.ok_or("Не указано начальное состояние")?;
        FA::new(
            self.states,
            self.alphabet,
            self.transitions,
            initial_state,
            self.final_states,
        )
    }
}
