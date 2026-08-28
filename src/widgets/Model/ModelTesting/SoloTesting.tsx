import Play from "@/assets/svg/Play.svg?react";
import FastForward from "@/assets/svg/FastForward.svg?react";
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
import { useCurrentTab } from "@/context/TabsContext.tsx";

interface Step {
    id: number;
    fromState: number;
    toState?: number;
    symbol: string;
    status: string;
}

export default function SoloTesting() {
    const [ testLine, setTestLine ] = useState<string>("");
    const [ isPlay ] = useState(false);
    const currentTab = useCurrentTab();
    const symbols = testLine.split("");

    const [ history ] = useState<Step[]>([]);
    const [ currentIndex ] = useState(0);
    const [ currentNode ] = useState<number | null>(null);
    const [ finalStatus ] = useState<string | null>(null);

    const handlePlay = () => {
        /*if (!testLine) return;

        const startNode = nodes.find((n) => n.isInitial)?.id;

        setCurrentNode(startNode ?? null);
        setCurrentIndex(0);
        setHistory([]);
        setFinalStatus(null);
        setIsPlay(true);*/
    };

    const handleStep = () => {
        /* if (currentNode === null || finalStatus) return;

        if (currentIndex >= symbols.length) {
            const isAccepted = nodes.find((n) => n.id === currentNode)?.isFinal;
            setFinalStatus(isAccepted ? "accepted" : "reject");
            return;
        }

        const symbol = symbols[currentIndex];

        const edge = edges.find(
            (e) =>
                e.idStartNode === currentNode &&
                e.state?.includes(symbol),
        );

        if (!edge) {
            setHistory((prev) => [
                ...prev,
                {
                    id: currentIndex,
                    fromState: currentNode,
                    toState: undefined,
                    symbol,
                    status: "reject",
                },
            ]);

            setSelectedEdge(null);
            setSelectedNode(currentNode);

            setFinalStatus("reject");
            return;
        }

        const nextNode = edge.idEndNode;
        setSelectedEdge(edge.id);
        setSelectedNode(nextNode);
        setHistory((prev) => [
            ...prev,
            {
                id: currentIndex,
                fromState: currentNode,
                toState: nextNode,
                symbol,
                status: "ok",
            },
        ]);

        setCurrentNode(nextNode);
        setCurrentIndex((prev) => prev + 1);*/
    };

    const handleFastForward = () => {
        /*let stepsGuard = 0;

        while (!finalStatus && stepsGuard < 1000) {
            stepsGuard++;
            handleStep();
        }*/
    };

    const handleReset = () => {
        /*setIsPlay(false);
        setHistory([]);
        setCurrentIndex(0);
        setCurrentNode(null);
        setFinalStatus(null);
        setSelectedNode(null);
        setSelectedEdge(null);*/
    };

    return (
        <>
            <Typography variant="pretitle">Входная строка</Typography>

            <Textfield
                value={ testLine }
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
                        <Button variant="control" onClick={ handleStep }>
                            <Forward />
                            Шаг
                        </Button>

                        <Button variant="control" onClick={ handleFastForward }>
                            <FastForward />
                            До конца
                        </Button>
                    </div>

                    <div style={ { display: "flex", flexDirection: "row" } }>
                        <Button variant="control" fullWidth onClick={ handleReset }>
                            <Reset />
                            Сброс
                        </Button>
                    </div>

                    <div className={ styles.wrapper }>
                        <div className={ styles.titleStepWrapper }>
                            <p className={ styles.positionLbl }>Позиция</p>
                            <p className={ styles.stepLbl }>
                                {currentIndex}/{testLine.length}
                            </p>
                        </div>

                        <div className={ styles.lineStateWrapper }>
                            {symbols.map((char, index) => {
                                const isOk = history[index]?.status === "ok";
                                const isCurrent = !history[index] && index === currentIndex;

                                return (
                                    <div
                                        key={ index }
                                        className={ clsx(
                                            styles.statesWrapper,
                                            history[index]
                                                ? (isOk ? styles.statesWrapperSuccess : styles.statesWrapperError)
                                                : (isCurrent ? styles.statesWrapperCurrent : ""),
                                        ) }
                                    >
                                        <p className={ clsx(
                                            styles.state,
                                            history[index]
                                                ? (isOk ? styles.stateSuccess : styles.stateError)
                                                : (isCurrent ? styles.stateCurrent : ""),
                                        ) }>{char}</p>
                                    </div>
                                );
                            })}
                        </div>

                        <div className={ styles.stateStatusWrapper }>
                            <p className={ styles.stateLbl }>
                                Состояние: {currentTab?.automaton.states.find((n) => n.id === currentNode)?.label}
                            </p>

                            <p
                                className={ clsx(
                                    styles.stateLbl,
                                    finalStatus === "accepted" && styles.stateLblAccepted,
                                    finalStatus === "reject" && styles.stateLblRejected,
                                ) }
                            >
                                {finalStatus === "accepted" && "Принято"}
                                {finalStatus === "reject" && "Отклонено"}
                            </p>
                        </div>
                    </div>

                    <div className={ styles.wrapper }>
                        <p className={ styles.positionLbl }>История</p>

                        {history.map((step, index) => {
                            const isLast = index === history.length - 1;
                            const isSuccess = step.status === "ok";

                            return (
                                <div
                                    key={ index }
                                    className={ clsx(
                                        styles.historyCard,
                                        isLast && (isSuccess ? styles.historyCardSuccess : styles.historyCardError),
                                    ) }
                                >
                                    <span className={ styles.historyActive }>{index + 1}.</span>

                                    <span className={ styles.historyState }>
                                        {currentTab?.automaton.states.find((n) => n.id === step.fromState)?.label}
                                        <ArrowRight />
                                        {step.toState !== undefined
                                            ? currentTab?.automaton.states.find((n) => n.id === step.toState)?.label
                                            : "—"}
                                        <ArrowRightToLine />
                                    </span>

                                    <span className={ styles.historySymbol }>
                                        {step.symbol}
                                    </span>
                                </div>
                            );
                        })}
                    </div>
                </>
            )}
        </>
    );
}
