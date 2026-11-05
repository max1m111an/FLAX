import styles from "./TraceCard.module.scss";
import clsx from "clsx";
import { AutomatonModel } from "@/types/Automaton.ts";
import ArrowRight from "@/assets/svg/ArrowRight.svg?react";
import ArrowRightToLine from "@/assets/svg/ArrowRightToLine.svg?react";
import { RunStepMealy, Trace } from "@/types/Testing.ts";
import { AutomatonType } from "@/types/Payloads.ts";

interface TraceCardProps {
    traceSteps: Trace;
    currentIndex: number;
    testLineLength: number;
    symbols: string[];
    currentAutomaton: AutomatonModel;
    type: AutomatonType,
}

export function TraceCard({
    traceSteps,
    currentIndex,
    testLineLength,
    symbols,
    currentAutomaton,
    type,
}: TraceCardProps) {

    const isMealy = type === "MEALY";
    const activeIndex = Math.min(currentIndex, traceSteps.steps.length);
    const currentStateId = activeIndex < traceSteps.steps.length
        ? traceSteps.steps[activeIndex]?.from
        : (traceSteps.steps.length > 0
            ? traceSteps.steps[traceSteps.steps.length - 1]?.to
            : currentAutomaton.states.find((s) => s.isInitial)?.id);

    const currentStateLabel = currentAutomaton.states.find((n) => n.id === currentStateId)?.label;
    const historyCount = Math.min(currentIndex, traceSteps.steps.length + 1);
    const showTraceStatus = isMealy
        ? currentIndex === symbols.length
        : currentIndex === traceSteps.steps.length;

    return (
        <div className={ styles.wrapper }>
            <div className={ styles.titleStepWrapper }>
                <p className={ styles.positionLbl }>Позиция</p>
                <p className={ styles.stepLbl }>
                    {currentIndex}/{testLineLength}
                </p>
            </div>

            <div className={ styles.lineStateWrapper }>
                {symbols.map((char, index) => {
                    const isActiveForTrace = index <= traceSteps.steps.length;
                    const isPassed = index < currentIndex && isActiveForTrace;
                    const isCurrent = index === currentIndex && isActiveForTrace;
                    const isOk = traceSteps.steps[index] !== undefined;
                    const step = traceSteps.steps[index] as RunStepMealy | undefined;
                    const output = isPassed ? (step?.output ?? "—") : "";

                    const statusClass = isPassed
                        ? (isOk ? styles.statesWrapperSuccess : styles.statesWrapperError)
                        : (isCurrent ? styles.statesWrapperCurrent : "");
                    const textClass = isPassed
                        ? (isOk ? styles.stateSuccess : styles.stateError)
                        : (isCurrent ? styles.stateCurrent : "");

                    return (
                        <div
                            key={ index }
                            className={ clsx(
                                styles.statesWrapper,
                                isMealy && styles.statesWrapperMealy,
                                statusClass,
                            ) }
                        >
                            <p className={ clsx(styles.state, textClass) }>
                                {char}
                            </p>
                            {isMealy && (
                                <p className={ clsx(styles.state, styles.stateOutput, textClass) }>
                                    {output || "\u00A0"}
                                </p>
                            )}
                        </div>
                    );
                })}
            </div>

            <div className={ styles.stateStatusWrapper }>
                <p className={ styles.stateLbl }>
                    Состояние: {currentStateLabel || "—"}
                </p>
                {showTraceStatus && (
                    <p
                        className={ clsx(
                            styles.stateLbl,
                            traceSteps.isFinal && styles.stateLblAccepted,
                            !traceSteps.isFinal && styles.stateLblRejected,
                        ) }
                    >
                        {traceSteps.isFinal ? "Принято" : "Отклонено"}
                    </p>
                )}
            </div>

            {historyCount > 0 && (
                <div className={ styles.wrapper }>
                    <p className={ styles.positionLbl }>История</p>

                    {Array.from({ length: historyCount }).map((_, index) => {
                        const step = traceSteps.steps[index];
                        const prevStep = index > 0 ? traceSteps.steps[index - 1] : undefined;
                        const isOk = step !== undefined;

                        const fromNodeId = step
                            ? step.from
                            : (prevStep ? prevStep.to : currentAutomaton.states.find((s) => s.isInitial)?.id);
                        const toNodeId = step ? step.to : undefined;
                        const mealyOutput = step ? (step as RunStepMealy).output : "—";

                        return (
                            <div
                                key={ index }
                                className={ clsx(
                                    styles.historyCard,
                                    isOk ? styles.historyCardSuccess : styles.historyCardError,
                                ) }
                            >
                                <span className={ styles.historyActive }>{index + 1}.</span>
                                {
                                    (type === "FA" || isMealy) && (
                                        <span className={ styles.historyState }>
                                            {currentAutomaton.states.find((n) => n.id === fromNodeId)?.label || "—"}
                                            <ArrowRight />
                                            {toNodeId !== undefined
                                                ? currentAutomaton.states.find((n) => n.id === toNodeId)?.label
                                                : "—"}
                                            <ArrowRightToLine />
                                        </span>
                                    )
                                }
                                { type === "FA" && (
                                    <span className={ styles.historySymbol }>
                                        {symbols[index]}
                                    </span>
                                )}
                                { type === "MEALY" && (
                                    <span className={ clsx(styles.historySymbol, styles.historyMealy) }>
                                        {symbols[index]}
                                        <ArrowRight />
                                        {mealyOutput}
                                    </span>
                                )}
                            </div>
                        );
                    })}
                </div>
            )}
        </div>
    );
}