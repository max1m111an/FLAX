import Play from "@/assets/svg/Play.svg?react";
import FastForward from "@/assets/svg/FastForward.svg?react";
import ArrowRight from "@/assets/svg/ArrowRight.svg?react";
import Forward from "@/assets/svg/Forward.svg?react";
import Reset from "@/assets/svg/Reset.svg?react";
import ArrowRightToLine from "@/assets/svg/ArrowRightToLine.svg?react";
import { useState } from "react";
import { useControl } from "@/context/ControlContext.tsx";

interface Step {
    id: number;
    fromState: number;
    toState?: number;
    symbol: string;
    status: string;
}

export default function ModelSoloTestingWidget() {
    const [ testLine, setTestLine ] = useState<string>("");
    const [ isPlay, setIsPlay ] = useState(false);

    const { nodes, edges, setSelectedEdge, setSelectedNode } = useControl();

    const symbols = testLine.split("");

    const [ history, setHistory ] = useState<Step[]>([]);
    const [ currentIndex, setCurrentIndex ] = useState(0);
    const [ currentNode, setCurrentNode ] = useState<number | null>(null);
    const [ finalStatus, setFinalStatus ] = useState<string | null>(null);

    const handlePlay = () => {
        if (!testLine) return;

        const startNode = nodes.find((n) => n.isInitial)?.id;

        setCurrentNode(startNode ?? null);
        setCurrentIndex(0);
        setHistory([]);
        setFinalStatus(null);
        setIsPlay(true);
    };

    const handleStep = () => {
        if (currentNode === null || finalStatus) return;

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
        setCurrentIndex((prev) => prev + 1);
    };

    const handleFastForward = () => {
        let stepsGuard = 0;

        while (!finalStatus && stepsGuard < 1000) {
            stepsGuard++;
            handleStep();
        }
    };

    const handleReset = () => {
        setIsPlay(false);
        setHistory([]);
        setCurrentIndex(0);
        setCurrentNode(null);
        setFinalStatus(null);
        setSelectedNode(null);
        setSelectedEdge(null);
    };

    return (
        <>
            <p className="model-pretitle">Входная строка</p>

            <input
                className="input"
                value={ testLine }
                onChange={ (e: React.ChangeEvent<HTMLInputElement>) => {
                    setTestLine(e.target.value);
                } }
            />

            {!isPlay && (
                <button className="main-btn" onClick={ handlePlay }>
                    <Play />
                    Запустить
                </button>
            )}

            {isPlay && (
                <>
                    <div className="model-step-fast-wrapper">
                        <button className="control-btn" onClick={ handleStep }>
                            <Forward />
                            Шаг
                        </button>

                        <button className="control-btn" onClick={ handleFastForward }>
                            <FastForward />
                            До конца
                        </button>
                    </div>

                    <div className="model-reset-btn-wrapper">
                        <button className="control-btn w-100" onClick={ handleReset }>
                            <Reset />
                            Сброс
                        </button>
                    </div>

                    <div className="test-state-wrapper">
                        <div className="title-step-wrapper">
                            <p className="position-lbl">Позиция</p>
                            <p className="step-lbl">
                                {currentIndex}/{testLine.length}
                            </p>
                        </div>

                        <div className="line-state-wrapper">
                            {symbols.map((char, index) => (
                                <div
                                    key={ index }
                                    className={ `states-wrapper ${
                                        history[index]
                                            ? history[index].status === "ok"
                                                ? "success"
                                                : "error"
                                            : index === currentIndex
                                                ? "current"
                                                : ""
                                    }` }
                                >
                                    <p className="state">{char}</p>
                                </div>
                            ))}
                        </div>

                        <div className="state-status-wrapper">
                            <p className="state-lbl">
                                Состояние: {nodes.find((n) => n.id === currentNode)?.name}
                            </p>

                            <p
                                className={ `state-lbl ${
                                    finalStatus === "accepted"
                                        ? "accepted"
                                        : finalStatus === "reject"
                                            ? "rejected"
                                            : ""
                                }` }
                            >
                                {finalStatus === "accepted" && "Принято"}
                                {finalStatus === "reject" && "Отклонено"}
                            </p>
                        </div>
                    </div>

                    <div className="test-state-wrapper">
                        <p className="position-lbl">История</p>

                        {history.map((step, index) => (
                            <div
                                key={ index }
                                className={ `history-card ${
                                    index === history.length - 1
                                        ? step.status === "ok"
                                            ? "success"
                                            : "error"
                                        : ""
                                }` }
                            >
                                <span className="history-active">{index + 1}.</span>

                                <span className="history-state">
                                    {nodes.find((n) => n.id === step.fromState)?.name}
                                    <ArrowRight />
                                    {step.toState !== undefined
                                        ? nodes.find((n) => n.id === step.toState)?.name
                                        : "—"}
                                    <ArrowRightToLine />
                                </span>

                                <span className="history-symbol">
                                    {step.symbol}
                                </span>
                            </div>
                        ))}
                    </div>
                </>
            )}
        </>
    );
}