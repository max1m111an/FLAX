import styles from "./TraceCard.module.scss";
import clsx from "clsx";
import { AutomatonModel } from "@/types/Automaton.ts";
import ArrowRight from "@/assets/svg/ArrowRight.svg?react";
import ArrowRightToLine from "@/assets/svg/ArrowRightToLine.svg?react";
import { RunStep, RunStepMealy, Trace } from "@/types/Testing.ts";
import { AutomatonType } from "@/types/Payloads.ts";
import { EPS } from "@/configs/CanvasConst.ts";

interface TraceCardProps {
    traceSteps: Trace;
    currentIndex: number;
    testLineLength: number;
    symbols: string[];
    currentAutomaton: AutomatonModel;
    type: AutomatonType;
}

/**
 * Карточка пошагового прогона одной ветки автомата.
 */
export function TraceCard({
    traceSteps,
    currentIndex,
    testLineLength,
    symbols,
    currentAutomaton,
    type,
}: TraceCardProps) {
    const isMealy = type === "MEALY";
    const steps = traceSteps.steps ?? [];
    const initialStateId = currentAutomaton.states.find((s) => s.isInitial)?.id;

    let totalRealConsumed = 0;
    let sliceEnd = 0;

    for (let i = 0; i < steps.length; i++) {
        if (steps[i].symbol !== EPS) {
            totalRealConsumed++;
        }
        if (totalRealConsumed <= currentIndex) {
            sliceEnd = i + 1;
        }
    }

    const visibleSteps: (RunStep | undefined)[] = currentIndex === 0
        ? []
        : steps.slice(0, sliceEnd);

    const isFailedAtCurrentStep = currentIndex > totalRealConsumed;
    if (isFailedAtCurrentStep) {
        visibleSteps.push(undefined);
    }

    const lastExecutedStep = steps[sliceEnd - 1];
    const currentStateId = lastExecutedStep ? lastExecutedStep.to : initialStateId;
    const currentStateLabel = currentAutomaton.states.find((n) => n.id === currentStateId)?.label;

    const isAtEnd = currentIndex >= symbols.length;
    const showTraceStatus = isFailedAtCurrentStep || isAtEnd;
    const isAccepted = !isFailedAtCurrentStep && isAtEnd && traceSteps.isFinal;

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
                    const isActiveForTrace = index <= totalRealConsumed;
                    const isPassed = index < currentIndex && isActiveForTrace;
                    const isCurrent = index === currentIndex && isActiveForTrace;
                    const isOk = index < totalRealConsumed;

                    const mealyStep = isMealy
                        ? steps.filter((s) => s.symbol !== EPS)[index] as RunStepMealy | undefined
                        : undefined;
                    const output = isPassed ? (mealyStep?.output ?? "—") : "";

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
                            isAccepted && styles.stateLblAccepted,
                            !isAccepted && styles.stateLblRejected,
                        ) }
                    >
                        {isAccepted ? "Принято" : "Отклонено"}
                    </p>
                )}
            </div>

            {visibleSteps.length > 0 && (
                <div className={ styles.wrapper }>
                    <p className={ styles.positionLbl }>История</p>

                    {visibleSteps.map((step, index) => {
                        const prevStep = index > 0 ? visibleSteps[index - 1] : undefined;
                        const isOk = step !== undefined;

                        const fromNodeId = step
                            ? step.from
                            : (prevStep ? prevStep.to : initialStateId);
                        const toNodeId = step ? step.to : undefined;
                        const symbolLabel = step ? step.symbol : symbols[totalRealConsumed];
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
                                {(type === "FA" || isMealy) && (
                                    <span className={ styles.historyState }>
                                        {currentAutomaton.states.find((n) => n.id === fromNodeId)?.label || "—"}
                                        <ArrowRight />
                                        {toNodeId !== undefined
                                            ? currentAutomaton.states.find((n) => n.id === toNodeId)?.label
                                            : "—"}
                                        <ArrowRightToLine />
                                    </span>
                                )}
                                {type === "FA" && (
                                    <span className={ styles.historySymbol }>
                                        {symbolLabel}
                                    </span>
                                )}
                                {type === "MEALY" && (
                                    <span className={ clsx(styles.historySymbol, styles.historyMealy) }>
                                        {symbolLabel}
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