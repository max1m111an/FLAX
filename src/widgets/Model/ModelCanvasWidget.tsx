import { StateNode } from "@/components/StateNode.tsx";
import { useCallback, useState } from "react";
import { EdgeState, useControl } from "@/context/ControlContext.tsx";
import Edge from "@/components/Edge";
import calculatePoints from "@/utils/calculatePoints.ts";
import { Textfield } from "@/components/ui/Textfield/Textfield.tsx";
import styles from "@/scenes/ModelScene.module.scss";
import { StateNodeModel } from "@/interface/Automaton.ts";

export default function ModelCanvasWidget() {
    const { activeControl, nodes, setNodes, edges, setEdges } = useControl();

    const [ tempEdge, setTempEdge ] = useState<{
        from: { x: number; y: number };
        to: { x: number; y: number };
    } | null>(null);


    const generateId = useCallback(() => Date.now(), []);

    const addNode = (e: React.MouseEvent<HTMLDivElement>) => {
        const x1 = e.clientX - 32;
        const y1 = e.clientY - 32;
        const nextIndex = nodes.length;
        const newNode: StateNodeModel = {
            id: generateId(),
            x: x1,
            y: y1,
            label: `q${nextIndex}`,
            is_initial: false,
            is_final: false,
        };
        setNodes([ ...nodes, newNode ]);
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

    const deleteNode = useCallback((id: number) => {
        setNodes((prev) => prev.filter((node) => node.id !== id));

        setEdges((prev) => prev.filter(
            (edge) => edge.idStartNode !== id && edge.idEndNode !== id,
        ));
    }, [ setNodes, setEdges ]);

    const deleteEdge = useCallback((id: number) => {
        setEdges((prev) => prev.filter((edge) => edge.id !== id));
    }, [ setEdges ]);

    return (
        <div
            className={ styles.modelCanvasWrapper }
            onClick={ activeControl === "node" ? addNode : undefined }
        >
            {nodes.map((node) => (
                <StateNode
                    label={ node.label }
                    initialPosition={ { x: node.x, y: node.y } }
                    isInitial={ node.is_initial }
                    isFinal={ node.is_final }
                    onStartEdge={ (pos) => setTempEdge({ from: pos, to: pos }) }
                    onMoveEdge={ (pos) =>
                        setTempEdge((prev) => prev && { ...prev, to: pos })
                    }
                    onEndEdge={ (hoveredNodeId) => addEdge(node.id, hoveredNodeId) }
                    key={ node.id }
                    id={ node.id }
                    onMoveNode={ (id, pos) => {
                        setNodes((prev) =>
                            prev.map((n) =>
                                n.id === id ? { ...n, x: pos.x, y: pos.y } : n,
                            ),
                        );
                    } }
                    onDeleteNode={ deleteNode }
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