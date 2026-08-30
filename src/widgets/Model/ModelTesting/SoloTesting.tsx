import Play from "@/assets/svg/Play.svg?react";
import FastForward from "@/assets/svg/FastForward.svg?react";
import Back from "@/assets/svg/Back.svg?react";
import ArrowRight from "@/assets/svg/ArrowRight.svg?react";
import Forward from "@/assets/svg/Forward.svg?react";
import Reset from "@/assets/svg/Reset.svg?react";
import ArrowRightToLine from "@/assets/svg/ArrowRightToLine.svg?react";
import { useState } from "react";
import { Button } from "@/components/ui/Button/Button.tsx";
import { Textfield } from "@/components/ui/Textfield/Textfield.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import clsx from "clsx";
import styles from "./SoloTesting.module.scss";
import { useCurrentTab, useTabs } from "@/context/TabsContext.tsx";
import type { TraceHighlight } from "@/context/TabsContext.tsx";
import { RunStep, runStrNFA } from "@/api/nfaAPI.ts";

export default function SoloTesting() {
    const [ testLine, setTestLine ] = useState<string>("");
    const [ isPlay, setIsPlay ] = useState(false);
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();
    const symbols = testLine.split("");
    const [ traces, setTraces ] = useState<{ steps: RunStep[]; isFinal: boolean }[]>([]);
    const [ currentIndex, setCurrentIndex ] = useState<number>(0);
    const [ finalStatus, setFinalStatus ] = useState<string | null>(null);

    if (!currentTab) return null;

    const applyHighlights = (consumed: number, activeTraces: { steps: RunStep[]; isFinal: boolean }[]) => {
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
                    if (
                        transition.from === step.from &&
                        transition.to === step.to &&
                        transition.symbol === step.symbol
                    ) {
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
        if (!testLine) return;

        const request = {
            automatonId: currentTab.id,
            input: testLine,
        };

        const response = await runStrNFA(request) as any;

        if ([ 200, 401, 402 ].includes(response.status)) {
            setTraces(response.traces || []);
            applyHighlights(0, response.traces || []);
        } else {
            setTraces([]);
            applyHighlights(0, []);
        }
        console.log(response);
        console.log(currentTab.automaton);
        setCurrentIndex(0);
        setFinalStatus(null);
        setIsPlay(true);
    };

    const handleStep = () => {
        if (currentIndex < symbols.length) {
            const nextIndex = currentIndex + 1;
            setCurrentIndex(nextIndex);
            applyHighlights(nextIndex, traces);

            if (nextIndex === symbols.length) {
                setFinalStatus(traces.some((t) => t.isFinal) ? "accepted" : "reject");
            }
        }
    };

    const handleStepBack = () => {
        if (currentIndex > 0) {
            const prevIndex = currentIndex - 1;
            setCurrentIndex(prevIndex);
            applyHighlights(prevIndex, traces);
            setFinalStatus(null);
        }
    };

    const handleFastForward = () => {
        setCurrentIndex(symbols.length);
        applyHighlights(symbols.length, traces);
        setFinalStatus(traces.some((t) => t.isFinal) ? "accepted" : "reject");
    };

    const handleReset = () => {
        setIsPlay(false);
        setCurrentIndex(0);
        setFinalStatus(null);
        setTraces([]);
        updateTab({ ...currentTab, selectedState: null, selectedTransition: null, selectedNodeId: null });
    };

    const displayTraces = traces.length > 0 ? traces : [ { steps: [], isFinal: false } ];

    return (
        <>
            <Typography variant="pretitle">Входная строка</Typography>

            <Textfield
                value={ testLine }
                disabled={ isPlay }
                onChange={ (e: React.ChangeEvent<HTMLInputElement>) => {
                    setTestLine(e.target.value);
                } }
            />

            {!isPlay && (
                <Button variant="main" onClick={ handlePlay }>
                    <Play />
                    Запустить
                </Button>
            )}

            {isPlay && (
                <>
                    <div style={ { display: "flex", flexDirection: "row", gap: "8px" } }>
                        <Button
                            variant="control"
                            onClick={ handleStep }
                            disabled={ currentIndex === symbols.length }
                        >
                            <Forward />
                            Шаг вперед
                        </Button>
                        <Button variant="control" onClick={ handleStepBack } disabled={ currentIndex === 0 }>
                            <Back />
                            Шаг назад
                        </Button>

                    </div>

                    <div style={ { display: "flex", flexDirection: "row", gap: "8px" } }>
                        <Button variant="control" fullWidth onClick={ handleFastForward } disabled={ currentIndex === symbols.length }>
                            <FastForward />
                            До конца
                        </Button>

                        <Button variant="control" fullWidth onClick={ handleReset }>
                            <Reset />
                            Сброс
                        </Button>
                    </div>

                    {displayTraces.map((traceObj, i) => {
                        const traceSteps = traceObj.steps || [];
                        const activeIndex = Math.min(currentIndex, traceSteps.length);

                        const currentNodeId = activeIndex < traceSteps.length
                            ? traceSteps[activeIndex]?.from
                            : (traceSteps.length > 0
                                ? traceSteps[traceSteps.length - 1]?.to
                                : currentTab.automaton.states.find((s) => s.isInitial)?.id);

                        const currentNodeLabel = currentTab.automaton.states.find((n) => n.id === currentNodeId)?.label;
                        const historyCount = Math.min(currentIndex, traceSteps.length + 1);

                        return (
                            <div key={ i } className={ styles.wrapper }>
                                <div className={ styles.titleStepWrapper }>
                                    <p className={ styles.positionLbl }>Позиция</p>
                                    <p className={ styles.stepLbl }>
                                        {currentIndex}/{testLine.length}
                                    </p>
                                </div>

                                <div className={ styles.lineStateWrapper }>
                                    {symbols.map((char, index) => {
                                        const isActiveForTrace = index <= traceSteps.length;
                                        const isPassed = index < currentIndex && isActiveForTrace;
                                        const isCurrent = index === currentIndex && isActiveForTrace;
                                        const isOk = traceSteps[index] !== undefined;

                                        return (
                                            <div
                                                key={ index }
                                                className={ clsx(
                                                    styles.statesWrapper,
                                                    isPassed
                                                        ? (isOk ? styles.statesWrapperSuccess : styles.statesWrapperError)
                                                        : (isCurrent ? styles.statesWrapperCurrent : ""),
                                                ) }
                                            >
                                                <p className={ clsx(
                                                    styles.state,
                                                    isPassed
                                                        ? (isOk ? styles.stateSuccess : styles.stateError)
                                                        : (isCurrent ? styles.stateCurrent : ""),
                                                ) }>
                                                    {char}
                                                </p>
                                            </div>
                                        );
                                    })}
                                </div>

                                <div className={ styles.stateStatusWrapper }>
                                    <p className={ styles.stateLbl }>
                                        Состояние: {currentNodeLabel || "—"}
                                    </p>
                                    {finalStatus !== null && (
                                        <p
                                            className={ clsx(
                                                styles.stateLbl,
                                                traceObj.isFinal && styles.stateLblAccepted,
                                                !traceObj.isFinal && styles.stateLblRejected,
                                            ) }
                                        >
                                            {traceObj.isFinal ? "Принято" : "Отклонено"}
                                        </p>
                                    )}
                                </div>

                                {historyCount > 0 && (
                                    <div className={ styles.wrapper }>
                                        <p className={ styles.positionLbl }>История</p>

                                        {Array.from({ length: historyCount }).map((_, index) => {
                                            const step = traceSteps[index];
                                            const prevStep = index > 0 ? traceSteps[index - 1] : undefined;
                                            const isOk = step !== undefined;

                                            const fromNodeId = step
                                                ? step.from
                                                : (prevStep ? prevStep.to : currentTab.automaton.states.find((s) => s.isInitial)?.id);
                                            const toNodeId = step ? step.to : undefined;

                                            return (
                                                <div
                                                    key={ index }
                                                    className={ clsx(
                                                        styles.historyCard,
                                                        isOk ? styles.historyCardSuccess : styles.historyCardError,
                                                    ) }
                                                >
                                                    <span className={ styles.historyActive }>{index + 1}.</span>

                                                    <span className={ styles.historyState }>
                                                        {currentTab.automaton.states.find((n) => n.id === fromNodeId)?.label || "—"}
                                                        <ArrowRight />
                                                        {toNodeId !== undefined
                                                            ? currentTab.automaton.states.find((n) => n.id === toNodeId)?.label
                                                            : "—"}
                                                        <ArrowRightToLine />
                                                    </span>

                                                    <span className={ styles.historySymbol }>
                                                        {symbols[index]}
                                                    </span>
                                                </div>
                                            );
                                        })}
                                    </div>
                                )}
                            </div>
                        );
                    })}
                </>
            )}
        </>
    );
}