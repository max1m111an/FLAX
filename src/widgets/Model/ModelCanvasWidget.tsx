import { State } from "@/components/State/State.tsx";
import React, { useCallback, useMemo, useRef, useState } from "react";
import Transition from "@/components/Edge/Transition.tsx";
import calculatePoints, { buildSelfLoop } from "@/utils/calculatePoints.ts";
import { Textfield } from "@/components/ui/Textfield/Textfield.tsx";
import styles from "@/scenes/ModelScene.module.scss";
import { useCurrentTab } from "@/context/TabsContext.tsx";
import { TransitionModel } from "@/types/Automaton.ts";
import { NODE_RADIUS, NODE_SIZE } from "@/configs/CanvasConst.ts";
import { ContextMenuInput, ContextMenuItem, useContextMenu } from "@/context/ContextMenuContext.tsx";
import Mark from "@/assets/svg/Mark.svg?react";
import { useStateActions } from "@/hooks/useStateAction.tsx";
import { AppTab } from "@/types/Tabs.ts";
import { useTransitionAction } from "@/hooks/useTransitionAction.tsx";


export default function ModelCanvasWidget() {
    const currentTab: AppTab | undefined = useCurrentTab();

    const [ tempEdge, setTempEdge ] = useState<{
        from: { x: number; y: number };
        to: { x: number; y: number };
    } | null>(null);
    const { setMenuState } = useContextMenu();
    const { handleFinalChange, handleInitialChange, handleNameChange, addState, removeState, saveStatePosition, moveStateLocal } = useStateActions(); //todo rename from handle to edit|update

    const { addTransition, removeTransition } = useTransitionAction();

    const [ draftEdge, setDraftEdge ] = useState<{
        from: number;
        to: number;
    } | null>(null);

    const canvasRef = useRef<HTMLDivElement | null>(null);

    const getCanvasRect = useCallback((): DOMRect => canvasRef.current!.getBoundingClientRect(), []);

    const transitions = currentTab?.type !== "SETTINGS" ? currentTab?.automaton.transitions : undefined;

    const { groupedTransitions, directedKeys } = useMemo(() => {
        if (!transitions) return { groupedTransitions: [], directedKeys: new Set<string>() };

        const groups = Object.values(
            transitions.reduce((acc, edge) => {
                const key = `${edge.from}-${edge.to}`;
                if (!acc[key]) acc[key] = [];
                acc[key].push(edge);
                return acc;
            }, {} as Record<string, typeof transitions>),
        ).map((group) => ({
            ...group[0],
            allSymbols: group.map((t) => t.symbol),
        }));

        const keys = new Set(transitions.map((t) => `${t.from}-${t.to}`));

        return { groupedTransitions: groups, directedKeys: keys };
    }, [ transitions ]);

    if (!currentTab || currentTab.type === "SETTINGS") return null;

    const clampToCanvas = (x: number, y: number) => {
        const rect = getCanvasRect();
        return {
            x: Math.max(0, Math.min(x, rect.width - NODE_SIZE)),
            y: Math.max(0, Math.min(y, rect.height - NODE_SIZE)),
        };

    };

    const handleAddState = (e: React.MouseEvent<HTMLDivElement>) => {
        const rect = getCanvasRect();
        const { x, y } = clampToCanvas(
            e.clientX - rect.left - NODE_RADIUS,
            e.clientY - rect.top - NODE_RADIUS,
        );

        addState(x, y);
    };

    const handleEdgeDrop = (startState: number, endState?: number) => {
        setTempEdge(null);

        if (endState === undefined) return;

        setDraftEdge({ from: startState, to: endState });
    };

    const handleAddTransition = async (startState: number, endState: number, symbolStr: string) => {
        if (!currentTab || currentTab.type === "SETTINGS") return;

        setDraftEdge(null);

        addTransition(startState, endState, symbolStr);
    };


    const CURVE = 40;

    const hasReverse = (from: number, to: number): boolean =>
        from !== to && directedKeys.has(`${to}-${from}`);


    const handleRightClickState = (e: React.MouseEvent, stateId: number) => {
        e.preventDefault();
        e.stopPropagation();
        const clickedState = currentTab.automaton.states.find((state) => state.id === stateId);

        if (!clickedState) return;

        setMenuState({
            visible: true,
            x: e.pageX,
            y: e.pageY,
            content: (
                <>
                    <ContextMenuItem
                        onClick={ (e) => e.stopPropagation() }
                        isInput
                    >
                        <ContextMenuInput
                            defaultValue={ clickedState.label }
                            onKeyDown={ (e) => {
                                if (e.key === "Enter") {
                                    handleNameChange(e.currentTarget.value, clickedState.id);
                                }
                            } }
                            onBlur={ (e) => {
                                handleNameChange(e.currentTarget.value, clickedState.id);
                            } }
                        />
                    </ContextMenuItem>
                    <ContextMenuItem onClick={ () => handleInitialChange(!clickedState.isInitial, clickedState.id) }>
                        Начальное
                        {clickedState.isInitial && <Mark />}
                    </ContextMenuItem>
                    <ContextMenuItem onClick={ () => handleFinalChange(!clickedState.isFinal, clickedState.id) }>
                        Финальное
                        {clickedState.isFinal && <Mark />}
                    </ContextMenuItem>
                    <ContextMenuItem onClick={ () => removeState(clickedState.id) } isRed>
                        Удалить
                    </ContextMenuItem>
                </>
            ),
        });
    };
    const handleRightClickTransition = (e: React.MouseEvent, transitionid: number) => {
        e.preventDefault();
        e.stopPropagation();
        const clickedTransition = currentTab.automaton.transitions.find((transition) => transition.id === transitionid);

        if (!clickedTransition) return;

        setMenuState({
            visible: true,
            x: e.pageX,
            y: e.pageY,
            content: (
                <>
                    <ContextMenuItem onClick={ () => removeTransition(clickedTransition.id) } isRed={ true }>
                        Удалить
                    </ContextMenuItem>
                </>
            ),
        });
    };

    return (
        <div
            ref={ canvasRef }
            className={ styles.modelCanvasWrapper }
            onClick={ (e) => {
                if (currentTab.activeControl === "node") {
                    handleAddState(e);
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
                    onDeleteNode={ () => removeState(state.id) }
                    onMoveNode={ moveStateLocal }
                    onEndMoveNode={ (id, pos) => saveStatePosition(id, pos.x, pos.y) }
                    getCanvasRect={ getCanvasRect }
                    onContextMenu={ (e) => handleRightClickState(e, state.id) }
                    highlightStatus={ currentTab.selectedState?.find((s) => s.id === state.id)?.status || null }
                    activeControl={ currentTab.activeControl }
                />
            ))}

            <svg style={ { position: "absolute", left: 0, top: 0, width: "100%", height: "100%", pointerEvents: "none" } }>

                {groupedTransitions.map((edgeGroup) => {
                    const relatedTransitionIds = (currentTab.automaton.transitions ?? [])
                        .filter((t) => edgeGroup.from !== undefined && edgeGroup.to !== undefined && t.from === edgeGroup.from && t.to === edgeGroup.to)
                        .map((t) => t.id);

                    const highlightData = currentTab.selectedTransition?.find((h) =>
                        relatedTransitionIds.includes(h.id),
                    );
                    const highlightStatus = highlightData ? highlightData.status : null;

                    const isSelected = !!(highlightData && !highlightData.status);


                    if (edgeGroup.from === edgeGroup.to) {
                        const state = currentTab.automaton.states.find((s) => s.id === edgeGroup.from);
                        if (!state) return null;
                        const loop = buildSelfLoop(state.x, state.y);
                        if (!loop) return null;

                        return (
                            <Transition
                                key={ edgeGroup.id }
                                id={ edgeGroup.id }
                                x1={ loop.textX }
                                y1={ loop.textY }
                                x2={ loop.textX }
                                y2={ loop.textY }
                                loopD={ loop.d }
                                textX={ loop.textX }
                                textY={ loop.textY }
                                angle={ 0 }
                                label={ edgeGroup.allSymbols.join("") }


                                activeControl={ currentTab.activeControl }
                                highlightStatus={ highlightStatus }
                                isSelected={ isSelected }
                                onMouseDown={ (edgeId) => {
                                    if (currentTab.activeControl === "trashcan") {
                                        removeTransition(edgeId);
                                    }
                                } }
                                onContextMenu={ (e) => handleRightClickTransition(e, edgeGroup.id) }
                            />
                        );
                    }

                    const reverse = hasReverse(edgeGroup.from, edgeGroup.to);
                    const bend = reverse ? -CURVE : 0;
                    const points = calculatePoints(edgeGroup, currentTab.automaton.states, bend);

                    if (!points) return null;

                    return (
                        <Transition
                            key={ edgeGroup.id }
                            id={ edgeGroup.id }
                            x1={ points.x1 }
                            y1={ points.y1 }
                            x2={ points.x2 }
                            y2={ points.y2 }
                            cx={ points.cx }
                            cy={ points.cy }
                            curved={ points.curved }
                            textX={ points.textX }
                            textY={ points.textY }
                            angle={ points.angle }
                            label={ edgeGroup.allSymbols.join("") }

                            activeControl={ currentTab.activeControl }
                            highlightStatus={ highlightStatus }
                            isSelected={ isSelected }
                            onMouseDown={ (edgeId) => {
                                if (currentTab.activeControl === "trashcan") {
                                    removeTransition(edgeId);
                                }
                            } }
                            onContextMenu={ (e) => handleRightClickTransition(e, edgeGroup.id) }
                        />
                    );
                })}

                {draftEdge && (() => {
                    if (draftEdge.from === draftEdge.to) {
                        const state = currentTab.automaton.states.find((s) => s.id === draftEdge.from);
                        if (!state) return null;

                        const loop = buildSelfLoop(state.x, state.y);
                        if (!loop) return null;
                        return (
                            <Transition
                                x1={ loop.textX }
                                y1={ loop.textY }
                                x2={ loop.textX }
                                y2={ loop.textY }
                                loopD={ loop.d }
                                activeControl={ currentTab.activeControl }
                            />
                        );
                    }

                    const fakeEdge = { id: -1, from: draftEdge.from, to: draftEdge.to, symbol: "" } as TransitionModel;
                    const points = calculatePoints(fakeEdge, currentTab.automaton.states);

                    if (!points) return null;
                    return (
                        <Transition
                            x1={ points.x1 }
                            y1={ points.y1 }
                            x2={ points.x2 }
                            y2={ points.y2 }
                            activeControl={ currentTab.activeControl }
                        />
                    );
                })()}

                {tempEdge && (
                    <Transition
                        x1={ tempEdge.from.x }
                        y1={ tempEdge.from.y }
                        x2={ tempEdge.to.x }
                        y2={ tempEdge.to.y }
                        activeControl={ currentTab.activeControl }
                    />
                )}
            </svg>

            {draftEdge && (() => {
                if (draftEdge.from === draftEdge.to) {
                    const state = currentTab.automaton.states.find((s) => s.id === draftEdge.from);
                    if (!state) return null;

                    const loop = buildSelfLoop(state.x, state.y);
                    if (loop) {
                        return (
                            <Textfield
                                key="draft-edge-input"
                                autoFocus
                                onEdge
                                style={ {
                                    position: "absolute",
                                    left: loop.textX,
                                    top: loop.textY,
                                    transform: "translate(-50%, -50%)",
                                } }
                                onKeyDown={ (e) => {
                                    if (e.key === "Enter") {
                                        handleAddTransition(draftEdge.from, draftEdge.to, (e.target as HTMLInputElement).value);
                                    } else if (e.key === "Escape") {
                                        setDraftEdge(null);
                                    }
                                } }
                                onBlur={ (e) => {
                                    const val = (e.target as HTMLInputElement).value;
                                    if (val.trim()) {
                                        handleAddTransition(draftEdge.from, draftEdge.to, val);
                                    } else {
                                        setDraftEdge(null);
                                    }
                                } }
                            />
                        );
                    }
                }

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
                            position: "absolute",
                            left: midX,
                            top: midY,
                            transform: "translate(-50%, -50%)",
                        } }
                        onKeyDown={ (e) => {
                            if (e.key === "Enter") {
                                handleAddTransition(draftEdge.from, draftEdge.to, (e.target as HTMLInputElement).value);
                            } else if (e.key === "Escape") {
                                setDraftEdge(null);
                            }
                        } }
                        onBlur={ (e) => {
                            const val = (e.target as HTMLInputElement).value;
                            if (val.trim()) {
                                handleAddTransition(draftEdge.from, draftEdge.to, val);
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
