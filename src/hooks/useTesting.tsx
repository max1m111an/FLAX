import { useEffect, useRef, useState } from "react";
import { useCurrentTab, useTabs } from "@/context/TabsContext.tsx";
import { modelServiceRegistry } from "@/services/ModelServiceRegistry.ts";
import { TraceHighlight } from "@/types/Tabs.ts";
import { RunStep, Trace } from "@/types/Testing.ts";
import { TransitionModel } from "@/types/Automaton.ts";
import { EPS } from "@/configs/CommonConst.ts";

/**
 * Находит все переходы на графе, соответствующие конкретному шагу трассы.
 */
const findMatchingTransitionIds = (
    transitions: TransitionModel[],
    step: RunStep,
): number[] =>
    transitions
        .filter((t) => {
            if (t.from !== step.from || t.to !== step.to) return false;
            const trigger = "input" in t ? t.input : t.symbol;
            return trigger === step.symbol;
        })
        .map((t) => t.id);


/**
 * Возвращает список уникальных веток для шага `consumed`.
 * Схлопывает ветки с одинаковым префиксом, показывает ветку в момент обрыва
 * и скрывает её на следующих шагах при наличии активных веток.
 */
export const getVisibleTraces = (
    traces: Trace[],
    consumed: number,
): Trace[] => {
    if (traces.length === 0) return [];
    if (consumed <= 0) return [ traces[0] ];

    const aliveOrJustFailed: { trace: Trace; key: string }[] = [];
    const earlierFailed: { trace: Trace; key: string }[] = [];

    for (const trace of traces) {
        const steps = trace.steps ?? [];
        let totalReal = 0;
        let targetIdx = -1;

        for (let i = 0; i < steps.length; i++) {
            if (steps[i].symbol !== EPS) {
                totalReal++;
            }
            if (totalReal === consumed) {
                targetIdx = i;
                while (i + 1 < steps.length && steps[i + 1].symbol === EPS) {
                    i++;
                    targetIdx = i;
                }
                break;
            }
        }

        const prefixSteps = targetIdx !== -1 ? steps.slice(0, targetIdx + 1) : steps;
        const prefixKey = prefixSteps.map((s) => `${s.from}->${s.to}:${s.symbol}`).join("|");

        const isReached = targetIdx !== -1;
        const justFailedHere = !isReached && totalReal === consumed - 1;

        if (isReached || justFailedHere) {
            const key = justFailedHere ? `${prefixKey}|FAIL@${consumed}` : prefixKey;
            aliveOrJustFailed.push({ trace, key });
        } else {
            earlierFailed.push({ trace, key: prefixKey });
        }
    }

    const pool = aliveOrJustFailed.length > 0 ? aliveOrJustFailed : earlierFailed;
    const uniqueMap = new Map<string, Trace>();

    for (const item of pool) {
        if (!uniqueMap.has(item.key)) {
            uniqueMap.set(item.key, item.trace);
        }
    }

    return Array.from(uniqueMap.values());
};


/**
 * Рассчитывает подсветку состояний и переходов для `consumed` поглощённых символов входной строки.
 * Корректно обрабатывает eps-переходы (`EPS`), не сдвигая индексы символов.
 */
const buildTraceHighlights = (
    activeTraces: { steps: RunStep[]; isFinal: boolean }[],
    consumed: number,
    totalSymbols: number,
    transitions: TransitionModel[],
    initialStateId?: number,
): { states: TraceHighlight[] | null; transitions: TraceHighlight[] | null } => {
    if (consumed <= 0) {
        return { states: null, transitions: null };
    }

    const stateMap = new Map<number, "success" | "error">();
    const transitionMap = new Map<number, "success" | "error">();
    const atEnd = consumed >= totalSymbols;

    for (const trace of activeTraces) {
        const steps = trace.steps ?? [];

        let realCount = 0;
        let targetIdx = -1;

        for (let i = 0; i < steps.length; i++) {
            if (steps[i].symbol !== EPS) {
                realCount++;
            }
            if (realCount === consumed) {
                targetIdx = i;
                while (i + 1 < steps.length && steps[i + 1].symbol === EPS) {
                    i++;
                    targetIdx = i;
                }
                break;
            }
        }

        const matched = targetIdx !== -1;
        const activeStep = matched ? steps[targetIdx] : steps[steps.length - 1];
        const stateId = activeStep ? activeStep.to : initialStateId;
        const status: "success" | "error" =
            !matched || (atEnd && !trace.isFinal) ? "error" : "success";

        if (stateId !== undefined && stateMap.get(stateId) !== "success") {
            stateMap.set(stateId, status);
        }

        if (activeStep) {
            for (const id of findMatchingTransitionIds(transitions, activeStep)) {
                if (transitionMap.get(id) !== "error") {
                    transitionMap.set(id, status);
                }
            }
        }
    }

    const toArray = (map: Map<number, "success" | "error">): TraceHighlight[] | null =>
        map.size > 0
            ? Array.from(map, ([ id, status ]) => ({ id, status }))
            : null;

    return {
        states: toArray(stateMap),
        transitions: toArray(transitionMap),
    };
};

/**
 * Хук управления пошаговым тестированием строки на активном автомате.
 */
export function useTesting () {
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();

    const [ testLine, setTestLine ] = useState<string>("");
    const [ isPlay, setIsPlay ] = useState(false);
    const [ traces, setTraces ] = useState<{ steps: RunStep[]; isFinal: boolean }[]>([]);
    const [ currentIndex, setCurrentIndex ] = useState<number>(0);
    const initializedTabIdRef = useRef<number | null>(null);

    const symbols = testLine.split("");
    const service = currentTab && currentTab.type !== "SETTINGS"
        ? modelServiceRegistry.getService(currentTab.type)
        : null;

    const applyHighlights = (consumed: number, activeTraces: { steps: RunStep[]; isFinal: boolean }[]) => {
        if (!currentTab || currentTab.type === "SETTINGS") return;

        const initialId = currentTab.automaton.states.find((s) => s.isInitial)?.id;
        const { states, transitions } = buildTraceHighlights(
            activeTraces,
            consumed,
            symbols.length,
            currentTab.automaton.transitions,
            initialId,
        );

        updateTab({
            ...currentTab,
            selectedState: states,
            selectedTransition: transitions,
            selectedNodeId: null,
        });
    };

    useEffect(() => {
        if (!currentTab || currentTab.type === "SETTINGS") return;
        if (initializedTabIdRef.current === currentTab.id) return;
        initializedTabIdRef.current = currentTab.id;

        if (currentTab.pendingTestLine && currentTab.pendingTraces && currentTab.pendingTraces.length > 0) {
            setTestLine(currentTab.pendingTestLine.split("\n")[0]);
            setTraces(currentTab.pendingTraces);
            setCurrentIndex(0);
            setIsPlay(true);
            applyHighlights(0, currentTab.pendingTraces);
            updateTab({
                ...currentTab,
                pendingTestLine: null,
                pendingTraces: null,
            });
        } else {
            setIsPlay(false);
            setCurrentIndex(0);
            setTraces([]);
            setTestLine(currentTab.testInput ? currentTab.testInput.split("\n")[0] : "");
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [ currentTab?.id ]);

    const handlePlay = async () => {
        if (!testLine || !service || !currentTab || currentTab.type === "SETTINGS") return;

        try {
            const response = await service.runStr(currentTab.automaton.id, testLine);
            const nextTraces = Array.isArray(response.traces) ? response.traces : [];
            setTraces(nextTraces);
            applyHighlights(0, nextTraces);
        } catch {
            setTraces([]);
            applyHighlights(0, []);
        }
        setCurrentIndex(0);
        setIsPlay(true);
    };

    const handleStep = () => {
        if (currentIndex < symbols.length) {
            const nextIndex = currentIndex + 1;
            setCurrentIndex(nextIndex);
            applyHighlights(nextIndex, traces);
        }
    };

    const handleStepBack = () => {
        if (currentIndex > 0) {
            const prevIndex = currentIndex - 1;
            setCurrentIndex(prevIndex);
            applyHighlights(prevIndex, traces);
        }
    };

    const handleFastForward = () => {
        setCurrentIndex(symbols.length);
        applyHighlights(symbols.length, traces);
    };

    const handleReset = () => {
        setIsPlay(false);
        setCurrentIndex(0);
        setTraces([]);
        if (currentTab && currentTab.type !== "SETTINGS") {
            updateTab({ ...currentTab, selectedState: null, selectedTransition: null, selectedNodeId: null });
        }
    };
    const visibleTraces = getVisibleTraces(traces, currentIndex);

    return {
        testLine,
        setTestLine,
        isPlay,
        currentIndex,
        symbols,
        traces: visibleTraces,
        currentTab,
        handlePlay,
        handleStep,
        handleStepBack,
        handleFastForward,
        handleReset,
    };
}