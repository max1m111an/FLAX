import { State } from "@/components/State.tsx";
import { useCallback, useState } from "react";
import { EdgeState, useControl } from "@/context/ControlContext.tsx";
import Edge from "@/components/Edge";
import calculatePoints from "@/utils/calculatePoints.ts";
import { Textfield } from "@/components/ui/Textfield/Textfield.tsx";
import styles from "@/scenes/ModelScene.module.scss";
import { tab, useTabs } from "@/context/TabsContext.tsx";
import { addStateNFA, removeStateNFA, updateStateNFA } from "@/api/nfaAPI.ts";

interface ModelCanvasWidgetProps {
    tab: tab;
}
export default function ModelCanvasWidget({ tab }: ModelCanvasWidgetProps) {
    const { activeControl, nodes, edges, setEdges } = useControl();
    const [ currentTab, setCurrentTab ] = useState<tab>(tab);
    const { updateTab } = useTabs();
    const [ tempEdge, setTempEdge ] = useState<{
        from: { x: number; y: number };
        to: { x: number; y: number };
    } | null>(null);


    const generateId = useCallback(() => Date.now(), []);

    const addNode = async (e: React.MouseEvent<HTMLDivElement>) => {
        const x1 = e.clientX - 32;
        const y1 = e.clientY - 32;
        const nextIndex = tab.automaton.states.length;
        const newNode = {
            automatonId: tab.id,
            label: `q${nextIndex}`,
            x: x1,
            y: y1,
            isInitial: false,
            isFinal: false,
        };
        const response = await addStateNFA(newNode);
        if (response.status == 200) {
            const newTabData = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    states: [ ...currentTab.automaton.states, response.state ],
                },
            };

            setCurrentTab(newTabData);
            updateTab(newTabData);
        }
    };

    const addEdge = (startNode: number, endNode?: number) => {
        setTempEdge(null);
        if (endNode === undefined) return;
        if (startNode === endNode) return;
        if (edges.find((edge) => (startNode === edge.idStartNode && endNode === edge.idEndNode
        ))) {
            return;
        }
        const newEdge: EdgeState = {
            id: generateId(),
            idStartNode: startNode,
            idEndNode: endNode,
            state: [],
            isEditing: true,
        };
        setEdges([ ...edges, newEdge ]);
    };

    const updateEdgeLabel = (id: number, state: string) => {
        if (state === "") {
            state = "λ";
        }
        const symbols = state.split(",").map((s) => s.trim()).filter((s) => s !== "");

        setEdges((prev) =>
            prev.map((edge) =>
                edge.id === id
                    ? { ...edge, state: symbols, isEditing: false }
                    : edge,
            ),
        );
    };
    const moveStateLocal = useCallback((id: number, pos: { x: number; y: number }) => {
        setCurrentTab((prev) => {
            if (!prev) return prev;
            return {
                ...prev,
                automaton: {
                    ...prev.automaton,
                    states: prev.automaton.states.map((state) =>
                        state.id === id ? { ...state, x: pos.x, y: pos.y } : state,
                    ),
                },
            };
        });
    }, []);
    const saveStatePosition = useCallback(async (id: number, pos: { x: number; y: number }) => {
        const request = {
            automatonId: tab.id,
            stateId: id,
            x: pos.x,
            y: pos.y,
        };

        try {
            const response = await updateStateNFA(request);
            if (response.status === 200) {
                setCurrentTab((prev) => {
                    const newTabData = {
                        ...prev,
                        automaton: {
                            ...prev.automaton,
                            states: prev.automaton.states.map((state) =>
                                state.id === id ? response.state : state,
                            ),
                        },
                    };
                    updateTab(newTabData);
                    return newTabData;
                });
            }
        } catch (error) {
            console.error("Ошибка при сохранении позиции вершины:", error);
        }
    }, [ tab.id, updateTab ]);

    const removeState = useCallback(async (id: number) => {
        const request = {
            automatonId: currentTab.id,
            stateId: id,
        };
        const response = await removeStateNFA(request);
        if (response.status == 200) {
            const newTabData = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    states: currentTab.automaton.states.filter((state) => state.id !== id),
                },
            };

            setCurrentTab(newTabData);
            updateTab(newTabData);
        }

    }, [ currentTab, updateTab ]);

    const deleteEdge = useCallback((id: number) => {
        setEdges((prev) => prev.filter((edge) => edge.id !== id));
    }, [ setEdges ]);

    return (
        <div
            className={ styles.modelCanvasWrapper }
            onClick={ activeControl === "node" ? addNode : undefined }
        >
            {currentTab.automaton.states.map((state) => (
                <State
                    label={ state.label }
                    initialPosition={ { x: state.x, y: state.y } }
                    isInitial={ state.is_initial }
                    isFinal={ state.is_final }
                    onStartEdge={ (pos) => setTempEdge({ from: pos, to: pos }) }
                    onMoveEdge={ (pos) =>
                        setTempEdge((prev) => prev && { ...prev, to: pos })
                    }
                    onEndEdge={ (hoveredNodeId) => addEdge(state.id, hoveredNodeId) }
                    key={ state.id }
                    id={ state.id }
                    onDeleteNode={ removeState }
                    onMoveNode={ moveStateLocal }
                    onEndMoveNode={ saveStatePosition }
                />
            ))}
            <svg style={ { position: "fixed", left: 0, top: 0, width: "100%", height: "100%", pointerEvents: "none" } }>
                {edges.map((edge) => {
                    const points = calculatePoints(edge, nodes);
                    if (!points) return null;

                    return (
                        <Edge
                            key={ edge.id }
                            id={ edge.id }
                            x1={ points.x1 }
                            y1={ points.y1 }
                            x2={ points.x2 }
                            y2={ points.y2 }
                            textX={ points.textX }
                            textY={ points.textY }
                            angle={ points.angle }
                            label={ edge.state }
                            isEditing={ edge.isEditing }
                            onDeleteEdge={ deleteEdge }
                        />
                    );
                })}
            </svg>
            {edges.map((edge) => {
                if (!edge.isEditing) return null;

                const points = calculatePoints(edge, nodes);
                if (!points) return null;

                const midX = (points.x1 + points.x2) / 2;
                const midY = (points.y1 + points.y2) / 2;

                return (
                    <Textfield
                        key={ `input-${edge.id}` }
                        autoFocus
                        onEdge
                        style={ {
                            position: "fixed",
                            left: midX,
                            top: midY,
                            transform: "translate(-50%, -50%)",
                        } }
                        onBlur={ (e) => updateEdgeLabel(edge.id, e.target.value) }
                        onKeyDown={ (e) => {
                            if (e.key === "Enter") {
                                updateEdgeLabel(edge.id, (e.target as HTMLInputElement).value);
                            }
                        } }
                    />
                );
            })}
            {tempEdge && (
                <svg style={ { position: "fixed", left: 0, top: 0, width: "100%", height: "100%", pointerEvents: "none" } }>
                    <Edge
                        x1={ tempEdge.from.x }
                        y1={ tempEdge.from.y }
                        x2={ tempEdge.to.x }
                        y2={ tempEdge.to.y }
                    />
                </svg>
            )}
        </div>
    );
}