import { State } from "@/components/State.tsx";
import { useState } from "react";
import Edge from "@/components/Edge";
import calculatePoints from "@/utils/calculatePoints.ts";
import { Textfield } from "@/components/ui/Textfield/Textfield.tsx";
import styles from "@/scenes/ModelScene.module.scss";
import { tab, useCurrentTab, useTabs } from "@/context/TabsContext.tsx";
import { addStateNFA, addTransitionNFA, removeStateNFA, removeTransitNFA, updateStateNFA } from "@/api/nfaAPI.ts";
import { TransitionModel } from "@/types/Automaton.ts";

export default function ModelCanvasWidget() {
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();
    const [ tempEdge, setTempEdge ] = useState<{
        from: { x: number; y: number };
        to: { x: number; y: number };
    } | null>(null);

    const [ draftEdge, setDraftEdge ] = useState<{
        from: number;
        to: number;
    } | null>(null);

    if (!currentTab) return null;

    const addNode = async (e: React.MouseEvent<HTMLDivElement>) => {
        const x1 = e.clientX - 32;
        const y1 = e.clientY - 32;
        const nextIndex = currentTab.automaton.states.length;
        const newNode = {
            automatonId: currentTab.id,
            label: `q${nextIndex}`,
            x: x1,
            y: y1,
            isInitial: false,
            isFinal: false,
        };
        const response = await addStateNFA(newNode);
        if (response.status == 200) {
            const newTabData: tab = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    states: [ ...currentTab.automaton.states, response.state ],
                },
            };
            updateTab(newTabData);
        }
    };

    const handleEdgeDrop = (startState: number, endState?: number) => {
        setTempEdge(null);

        if (endState === undefined) return;
        if (startState === endState) return;

        setDraftEdge({ from: startState, to: endState });
    };

    const addEdge = async (startState: number, endState: number, symbolStr: string) => {
        setDraftEdge(null); // Закрываем инпут

        let symbols = symbolStr.split(",").map((s) => s.trim()).filter((s) => s !== "");
        if (symbols.length === 0) {
            symbols = [ "λ" ];
        }

        const newTransition = {
            automatonId: currentTab.id,
            from: startState,
            to: endState,
            symbols,
        };

        const response = await addTransitionNFA(newTransition);
        if (response.status == 200) {
            const newTabData: tab = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    transitions: [ ...currentTab.automaton.transitions, ...response.transition ],
                },
            };
            updateTab(newTabData);
        }
    };

    const moveStateLocal = (id: number, pos: { x: number; y: number }) => {
        updateTab({
            ...currentTab,
            automaton: {
                ...currentTab.automaton,
                states: currentTab.automaton.states.map((state) =>
                    state.id === id ? { ...state, x: pos.x, y: pos.y } : state,
                ),
            },
        });
    };

    const saveStatePosition = async (id: number, pos: { x: number; y: number }) => {
        const request = {
            automatonId: currentTab.id,
            stateId: id,
            x: pos.x,
            y: pos.y,
        };

        try {
            const response = await updateStateNFA(request);
            if (response.status === 200) {
                const newTabData: tab = {
                    ...currentTab,
                    automaton: {
                        ...currentTab.automaton,
                        states: currentTab.automaton.states.map((state) =>
                            state.id === id ? response.state : state,
                        ),
                    },
                };
                updateTab(newTabData);
            }
        } catch (error) {
            console.error("Ошибка при сохранении позиции вершины:", error);
        }
    };

    const removeState = async (id: number) => {
        const connectedTransitions = currentTab.automaton.transitions.filter(
            (t) => t.from === id || t.to === id,
        );

        try {
            await Promise.all(
                connectedTransitions.map((t) =>
                    removeTransitNFA({ automatonId: currentTab.id, transitionId: t.id }),
                ),
            );

            const request = {
                automatonId: currentTab.id,
                stateId: id,
            };
            const response = await removeStateNFA(request);
            if (response.status == 200) {
                const connectedIds = connectedTransitions.map((t) => t.id);
                const newTabData = {
                    ...currentTab,
                    selectedNodeId: currentTab.selectedNodeId === id ? null : currentTab.selectedNodeId,
                    automaton: {
                        ...currentTab.automaton,
                        states: currentTab.automaton.states.filter((state) => state.id !== id),
                        transitions: currentTab.automaton.transitions.filter(
                            (t) => !connectedIds.includes(t.id),
                        ),
                    },
                };
                updateTab(newTabData);
            }
        } catch (error) {
            console.error("Ошибка при удалении состояния:", error);
        }
    };

    const groupedTransitions = Object.values(
        currentTab.automaton.transitions.reduce((acc, edge) => {
            const key = `${edge.from}-${edge.to}`;
            if (!acc[key]) {
                acc[key] = { ...edge, allSymbols: [ edge.symbol ] };
            } else {
                acc[key].allSymbols.push(edge.symbol);
            }
            return acc;
        }, {} as Record<string, TransitionModel & { allSymbols: string[] }>),
    );

    const removeEdge = async (id: number) => {
        const targetTransition = currentTab.automaton.transitions.find((t) => t.id === id);
        if (!targetTransition) return;

        const transitionsToDelete = currentTab.automaton.transitions.filter(
            (t) => t.from === targetTransition.from && t.to === targetTransition.to,
        );

        try {
            await Promise.all(
                transitionsToDelete.map((t) =>
                    removeTransitNFA({ automatonId: currentTab.id, transitionId: t.id }),
                ),
            );

            const idsToDelete = transitionsToDelete.map((t) => t.id);
            const newTabData = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    transitions: currentTab.automaton.transitions.filter(
                        (t) => !idsToDelete.includes(t.id),
                    ),
                },
            };

            updateTab(newTabData);
        } catch (error) {
            console.error("Ошибка при удалении ребра:", error);
        }
    };

    return (
        <div
            className={ styles.modelCanvasWrapper }
            onClick={ (e) => {
                if (currentTab.activeControl === "cursor") {
                    if (currentTab.selectedNodeId !== null) {
                        e.stopPropagation();
                        updateTab({ ...currentTab, selectedNodeId: null });
                    }
                } else if (currentTab.activeControl === "node") {
                    addNode(e);
                }
            } }
        >
            {currentTab.automaton.states.map((state) => (
                <State
                    label={ state.label }
                    initialPosition={ { x: state.x, y: state.y } }
                    isInitial={ state.isInitial }
                    isFinal={ state.isFinal }
                    onStartEdge={ (pos) => setTempEdge({ from: pos, to: pos }) }
                    onMoveEdge={ (pos) =>
                        setTempEdge((prev) => prev && { ...prev, to: pos })
                    }
                    onEndEdge={ (hoveredNodeId) => handleEdgeDrop(state.id, hoveredNodeId) }
                    key={ state.id }
                    id={ state.id }
                    onDeleteNode={ removeState }
                    onMoveNode={ moveStateLocal }
                    onEndMoveNode={ saveStatePosition }
                />
            ))}

            <svg style={ { position: "fixed", left: 0, top: 0, width: "100%", height: "100%", pointerEvents: "none" } }>

                {groupedTransitions.map((edgeGroup) => {
                    const points = calculatePoints(edgeGroup, currentTab.automaton.states);
                    if (!points) return null;

                    return (
                        <Edge
                            key={ edgeGroup.id }
                            id={ edgeGroup.id }
                            from={ edgeGroup.from }
                            to={ edgeGroup.to }
                            x1={ points.x1 }
                            y1={ points.y1 }
                            x2={ points.x2 }
                            y2={ points.y2 }
                            textX={ points.textX }
                            textY={ points.textY }
                            angle={ points.angle }
                            label={ edgeGroup.allSymbols.join(", ") }
                            onDeleteEdge={ removeEdge }
                        />
                    );
                })}

                {draftEdge && (() => {
                    const fakeEdge = { id: -1, from: draftEdge.from, to: draftEdge.to, symbol: "" } as TransitionModel;
                    const points = calculatePoints(fakeEdge, currentTab.automaton.states);

                    if (!points) return null;
                    return (
                        <Edge
                            x1={ points.x1 }
                            y1={ points.y1 }
                            x2={ points.x2 }
                            y2={ points.y2 }
                        />
                    );
                })()}

                {tempEdge && (
                    <Edge
                        x1={ tempEdge.from.x }
                        y1={ tempEdge.from.y }
                        x2={ tempEdge.to.x }
                        y2={ tempEdge.to.y }
                    />
                )}
            </svg>

            {draftEdge && (() => {
                const fakeEdge = { id: -1, from: draftEdge.from, to: draftEdge.to, symbol: "" } as TransitionModel;
                const points = calculatePoints(fakeEdge, currentTab.automaton.states);

                if (!points) return null;

                const midX = (points.x1 + points.x2) / 2;
                const midY = (points.y1 + points.y2) / 2;

                return (
                    <Textfield
                        key="draft-edge-input"
                        autoFocus
                        onEdge
                        style={ {
                            position: "fixed",
                            left: midX,
                            top: midY,
                            transform: "translate(-50%, -50%)",
                            zIndex: 1000,
                        } }
                        onKeyDown={ (e) => {
                            if (e.key === "Enter") {
                                addEdge(draftEdge.from, draftEdge.to, (e.target as HTMLInputElement).value);
                            } else if (e.key === "Escape") {
                                setDraftEdge(null);
                            }
                        } }
                        onBlur={ (e) => {
                            const val = (e.target as HTMLInputElement).value;
                            if (val.trim()) {
                                addEdge(draftEdge.from, draftEdge.to, val);
                            } else {
                                setDraftEdge(null);
                            }
                        } }
                    />
                );
            })()}
        </div>
    );
}