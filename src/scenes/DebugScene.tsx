import { addStateNFA, addTransitionNFA, createNewNFA, removeStateNFA, updateStateNFA } from "@/api/nfaAPI.ts";
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
                    const res = await createNewNFA("Test");
                    setResponse(`Успех:\n${JSON.stringify(res, null, 4)}`);
                } catch (e) {
                    setResponse(`Ошибка:\n${JSON.stringify(e, null, 4)}`);
                }
            } }>create_new_nfa</button>
            <button className={ styles.controlButton } onClick={ async () => {
                try {
                    const res = await addStateNFA({
                        automatonId: 2,
                        label: "q0",
                        x: 5455,
                        y: 555,
                        isInitial: false,
                        isFinal: false,
                    });
                    setResponse(`Успех:\n${JSON.stringify(res, null, 4)}`);
                } catch (e) {
                    setResponse(`Ошибка:\n${JSON.stringify(e, null, 4)}`);
                }
            } }>nfa_add_state</button>
            <button className={ styles.controlButton } onClick={ async () => {
                try {
                    const res = await removeStateNFA({
                        automatonId: 2,
                        stateId: 8335576,
                    });
                    setResponse(`Успех:\n${JSON.stringify(res, null, 4)}`);
                } catch (e) {
                    setResponse(`Ошибка:\n${JSON.stringify(e, null, 4)}`);
                }
            } }>nfa_delete_state</button>
            <button className={ styles.controlButton } onClick={ async () => {
                try {
                    const res = await updateStateNFA({
                        automatonId: 2,
                        stateId: 3159668,
                        label: "q4",
                        isFinal: true,
                    });
                    setResponse(`Успех:\n${JSON.stringify(res, null, 4)}`);
                } catch (e) {
                    setResponse(`Ошибка:\n${JSON.stringify(e, null, 4)}`);
                }
            } }>nfa_update_state</button>
            <button className={ styles.controlButton } onClick={ async () => {
                try {
                    const res = await addTransitionNFA({
                        automatonId: 2,
                        from: 3370849,
                        to: 5793196,
                        symbols: [ "d", "e", "f" ],
                    });
                    setResponse(`Успех:\n${JSON.stringify(res, null, 4)}`);
                } catch (e) {
                    setResponse(`Ошибка:\n${JSON.stringify(e, null, 4)}`);
                }
            } }>nfa_add_transition</button>
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

                    const statePositions = [
                        { x: 100, y: 100 },
                        { x: 100, y: 400 },
                        { x: 400, y: 100 },
                        { x: 400, y: 400 },
                        { x: 650, y: 400 },
                    ];

                    // 2. Массивы для сбора данных, чтобы потом отдать их React'у
                    const addedStates = [];
                    const addedTransitions = [];
                    const stateIds: number[] = [];

                    // Создаем вершины
                    for (let i = 0; i < 5; i++) {
                        const pos = statePositions[i];
                        const res = await addStateNFA({
                            automatonId,
                            label: `q${i}`,
                            x: pos.x,
                            y: pos.y,
                            isInitial: i === 0,
                            isFinal: i === 4,
                        });
                        if (res.status !== 200) throw new Error(`addState ${i} failed`);

                        stateIds.push(res.state.id);
                        addedStates.push(res.state); // Сохраняем вершину
                    }

                    // Создаем переходы
                    for (let i = 0; i < 4; i++) {
                        const res = await addTransitionNFA({
                            automatonId,
                            from: stateIds[i],
                            to: stateIds[i + 1],
                            symbols: [ String(i + 1) ],
                        });
                        if (res.status !== 200) throw new Error(`addTransition ${i} failed`);

                        addedTransitions.push(...res.transition); // Сохраняем переходы
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