import { useEffect, useRef, useState } from "react";
import { useCurrentTab, useTabs } from "@/context/TabsContext.tsx";
import { modelServiceRegistry } from "@/services/ModelServiceRegistry.ts";
import { TraceHighlight } from "@/types/Tabs.ts";
import { RunStep } from "@/types/Testing.ts"; // Ваш новый тип с дженериком

const isMatchingTransition = (transition: any, step: RunStep) => {
    if (transition.from !== step.from || transition.to !== step.to) return false;
    const trigger = "input" in transition ? transition.input : transition.symbol;
    return trigger === step.symbol;
};

export function useTesting() {
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();

    const [ testLine, setTestLine ] = useState<string>("");
    const [ isPlay, setIsPlay ] = useState(false);
    const [ traces, setTraces ] = useState<{ steps: RunStep[]; isFinal: boolean }[]>([]);
    const [ currentIndex, setCurrentIndex ] = useState<number>(0);
    const initializedRef = useRef(false);

    const symbols = testLine.split("");
    const service = currentTab && currentTab.type !== "SETTINGS" ? modelServiceRegistry.getService(currentTab.type) : null;

    useEffect(() => {
        if (!currentTab || currentTab.type === "SETTINGS") return;
        if (initializedRef.current) return;
        initializedRef.current = true;

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
        } else if (currentTab.testInput) {
            setTestLine(currentTab.testInput.split("\n")[0]);
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [ currentTab ]);

    const applyHighlights = (consumed: number, activeTraces: { steps: RunStep[]; isFinal: boolean }[]) => {
        if (!currentTab || currentTab.type === "SETTINGS") return;

        if (consumed <= 0) {
            updateTab({
                ...currentTab,
                selectedState: null,
                selectedTransition: null,
                selectedNodeId: null,
            });
            return;
        }

        const initialId = currentTab.automaton.states.find((s) => s.isInitial)?.id;
        const states: TraceHighlight[] = [];
        const transitions: TraceHighlight[] = [];

        for (const trace of activeTraces) {
            const steps = trace.steps || [];
            const hasConsumedStep = steps.length >= consumed;
            const atEnd = consumed >= symbols.length;

            const stateId = hasConsumedStep
                ? steps[consumed - 1].to
                : steps.length > 0
                    ? steps[steps.length - 1].to
                    : initialId;

            const status: "success" | "error" = !hasConsumedStep || (atEnd && !trace.isFinal) ? "error" : "success";

            if (stateId !== undefined) {
                const existing = states.find((s) => s.id === stateId);
                if (existing) {
                    if (status === "success") existing.status = "success";
                } else {
                    states.push({ id: stateId, status });
                }
            }

            const step = hasConsumedStep
                ? steps[consumed - 1]
                : (steps.length > 0 ? steps[steps.length - 1] : undefined);

            if (step) {
                for (const transition of currentTab.automaton.transitions) {
                    if (isMatchingTransition(transition, step)) {
                        const existing = transitions.find((t) => t.id === transition.id);
                        if (existing) {
                            if (status === "error") existing.status = "error";
                        } else {
                            transitions.push({ id: transition.id, status });
                        }
                    }
                }
            }
        }

        updateTab({
            ...currentTab,
            selectedState: states.length > 0 ? states : null,
            selectedTransition: transitions.length > 0 ? transitions : null,
            selectedNodeId: null,
        });
    };

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

    return {
        testLine,
        setTestLine,
        isPlay,
        currentIndex,
        symbols,
        traces,
        currentTab,
        handlePlay,
        handleStep,
        handleStepBack,
        handleFastForward,
        handleReset,
    };
}