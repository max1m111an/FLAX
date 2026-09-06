import { addState, addTransition } from "@/services/nfaService.ts";
import styles from "@/scenes/MainScene.module.scss";
import { useState } from "react";
import { useTabs } from "@/context/TabsContext.tsx";
import CircleDot from "@/assets/svg/CircleDot.svg?react";

export const DebugScene = () => {
    const [ response, setResponse ] = useState("");
    const { addTab, updateTab } = useTabs();
    return (
        <div>
            <button className={ styles.controlButton } onClick={ async () => {
                try {
                    const newTab = await addTab({
                        id: 0,
                        type: "Конечный автомат",
                        icon: CircleDot,
                        description: "Моделирование НКА, ДКА",
                    });

                    if (!newTab) throw new Error("Не удалось создать вкладку");

                    const automatonId = newTab.id;

                    const statesData = [
                        { label: "q0", x: 100, y: 100, isInitial: true, isFinal: false },
                        { label: "q1", x: 275, y: 100, isInitial: false, isFinal: false },
                        { label: "q2", x: 450, y: 100, isInitial: false, isFinal: false },
                        { label: "q3", x: 625, y: 100, isInitial: false, isFinal: false },
                        { label: "q4", x: 625, y: 250, isInitial: false, isFinal: true },
                        { label: "q5", x: 100, y: 250, isInitial: false, isFinal: false },
                        { label: "q6", x: 100, y: 400, isInitial: false, isFinal: false },
                        { label: "q7", x: 275, y: 250, isInitial: false, isFinal: false },
                        { label: "q8", x: 275, y: 400, isInitial: false, isFinal: false },
                        { label: "q9", x: 450, y: 250, isInitial: false, isFinal: false },
                        { label: "q10", x: 450, y: 400, isInitial: false, isFinal: false },
                        { label: "q11", x: 450, y: 550, isInitial: false, isFinal: true },
                        { label: "q12", x: 100, y: 550, isInitial: false, isFinal: false },
                        { label: "q13", x: 100, y: 675, isInitial: false, isFinal: false },
                        { label: "q14", x: 275, y: 675, isInitial: false, isFinal: false },
                    ];

                    const transitionsData = [
                        { from: 0, to: 1, symbol: "1" },
                        { from: 1, to: 2, symbol: "2" },
                        { from: 2, to: 3, symbol: "3" },
                        { from: 3, to: 4, symbol: "4" },
                        { from: 0, to: 5, symbol: "1" },
                        { from: 0, to: 5, symbol: "2" },
                        { from: 5, to: 6, symbol: "2" },
                        { from: 6, to: 12, symbol: "3" },
                        { from: 12, to: 13, symbol: "4" },
                        { from: 1, to: 7, symbol: "2" },
                        { from: 7, to: 8, symbol: "3" },
                        { from: 2, to: 9, symbol: "3" },
                        { from: 9, to: 10, symbol: "4" },
                        { from: 10, to: 11, symbol: "5" },
                        { from: 13, to: 14, symbol: "5" },
                        { from: 10, to: 4, symbol: "5" },
                    ];

                    const addedStates = [];
                    const addedTransitions = [];
                    const stateIds: number[] = [];

                    for (const data of statesData) {
                        const res = await addState({
                            automatonId,
                            label: data.label,
                            x: data.x,
                            y: data.y,
                            isInitial: data.isInitial,
                            isFinal: data.isFinal,
                        });

                        stateIds.push(res.state.id);
                        addedStates.push(res.state);
                    }

                    for (const data of transitionsData) {
                        const res = await addTransition({
                            automatonId,
                            from: stateIds[data.from],
                            to: stateIds[data.to],
                            symbols: [ data.symbol ],
                        });

                        addedTransitions.push(...res.transition);
                    }

                    updateTab({
                        ...newTab,
                        automaton: {
                            ...newTab.automaton,
                            states: addedStates,
                            transitions: addedTransitions,
                        },
                    });

                    console.log(`Успех: создан автомат #${automatonId} с состояниями ${stateIds.join(", ")}`);
                } catch (e) {
                    setResponse(`Ошибка:\n${JSON.stringify(e, null, 4)}`);
                }
            } }>create_chain</button>

            <pre
                className={ styles.cardDescriptionType }
                style={ {
                    backgroundColor: "#1e1e1e",
                    color: "#d4d4d4",
                    padding: "15px",
                    borderRadius: "8px",
                    overflowX: "auto",
                    fontFamily: "Consolas, monospace",
                    whiteSpace: "pre-wrap",
                    border: "1px solid #333",
                    userSelect: "text",
                } }
            >
                {response || "Ожидание ответа от бэкенда..."}
            </pre>
        </div>
    );
};