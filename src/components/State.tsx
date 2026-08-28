import React, { useLayoutEffect, useRef } from "react";
import ChevronsRight from "@/assets/svg/ChevronsRight.svg?react";
import clsx from "clsx";
import styles from "./StateNode.module.scss";
import { useCurrentTab, useTabs } from "@/context/TabsContext.tsx";

interface StateProps {
    isFinal?: boolean;
    isInitial?: boolean;
    label: string;
    initialPosition?: { x: number; y: number };
    onStartEdge?: (pos: { x: number; y: number }) => void;
    onMoveEdge?: (pos: { x: number; y: number }) => void;
    onEndEdge?: (hoveredNodeId: number | undefined) => void;
    id: number;
    onMoveNode?: (id: number, pos: { x: number; y: number }) => void;
    onEndMoveNode?: (id: number, pos: { x: number; y: number }) => void;
    onDeleteNode?: (id: number) => void;
}

export function State(
    {
        isFinal,
        isInitial,
        label,
        initialPosition = { x: 0, y: 0 },
        onStartEdge,
        onMoveEdge,
        onEndEdge,
        id,
        onMoveNode,
        onDeleteNode,
        onEndMoveNode,
    }: StateProps) {
    const nodeRef = useRef<HTMLDivElement | null>(null);
    const position = useRef(initialPosition);
    const offset = useRef({ x: 0, y: 0 });
    const dragging = useRef(false);
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();

    const { x: initialX, y: initialY } = initialPosition;

    useLayoutEffect(() => {
        if (!dragging.current && nodeRef.current) {
            position.current = { x: initialX, y: initialY };
            nodeRef.current.style.left = `${initialX}px`;
            nodeRef.current.style.top = `${initialY}px`;
        }
    }, [ initialX, initialY ]);

    const getCursorStyle = () => {
        if (currentTab?.activeControl === "move") {
            return dragging ? "grabbing" : "grab";
        }
        return "default";
    };

    const getZIndex = () => {
        return dragging ? 900 : "auto";
    };
    const onMouseDown = (e: React.MouseEvent) => {
        e.stopPropagation();
        e.preventDefault();
        if (currentTab?.activeControl === "move") {
            dragging.current = true;

            if (nodeRef.current) {
                const rect = nodeRef.current.getBoundingClientRect();
                offset.current = {
                    x: e.clientX - rect.left,
                    y: e.clientY - rect.top,
                };
            }

            document.addEventListener("mousemove", onMouseMove);
            document.addEventListener("mouseup", onMouseUp);
            return;
        }
        if (currentTab?.activeControl === "edge") {
            document.addEventListener("mousemove", onMouseMove);
            document.addEventListener("mouseup", onMouseUp);
            return;
        }
        if (currentTab?.activeControl === "trashcan") {
            onDeleteNode?.(id);
            return;
        }

    };
    const onMouseMove = (e: MouseEvent) => {
        e.stopPropagation();
        e.preventDefault();
        if (currentTab?.activeControl === "edge") {
            const rect = nodeRef.current!.getBoundingClientRect();

            const cx = rect.left + rect.width / 2;
            const cy = rect.top + rect.height / 2;

            const distToCenter = Math.sqrt(
                Math.pow(e.clientX - cx, 2) +
                Math.pow(e.clientY - cy, 2),
            );

            const radius = rect.width / 2;

            if (distToCenter < radius + 10) {
                return;
            }

            const dx = e.clientX - cx;
            const dy = e.clientY - cy;

            const len = Math.sqrt(dx * dx + dy * dy) || 1;
            const r = rect.width / 2;

            const start = {
                x: cx + (dx / len) * r,
                y: cy + (dy / len) * r,
            };

            onStartEdge?.(start);
            onMoveEdge?.({
                x: e.clientX,
                y: e.clientY,
            });
        }
        if (currentTab?.activeControl === "move") {
            if (!dragging.current) return;

            let newX = e.clientX - offset.current.x;
            let newY = e.clientY - offset.current.y;

            if (nodeRef.current) {
                const rect = nodeRef.current.getBoundingClientRect();
                newX = Math.max(0, Math.min(newX, window.innerWidth - rect.width));
                newY = Math.max(0, Math.min(newY, window.innerHeight - rect.height));
            }

            position.current = { x: newX, y: newY };

            if (nodeRef.current) {
                nodeRef.current.style.left = `${newX}px`;
                nodeRef.current.style.top = `${newY}px`;
            }
            onMoveNode?.(id, { x: newX, y: newY });
        }
    };
    const onMouseUp = (e: MouseEvent) => {
        e.stopPropagation();
        e.preventDefault();
        if (currentTab?.activeControl === "edge") {
            const element = document.elementFromPoint(e.clientX, e.clientY);
            const hoveredNode = element?.closest("[data-node-id]");
            const hoveredId = hoveredNode ? Number(hoveredNode.getAttribute("data-node-id")) : undefined;

            onEndEdge?.(hoveredId);

            document.removeEventListener("mousemove", onMouseMove);
            document.removeEventListener("mouseup", onMouseUp);
        }
        if (currentTab?.activeControl === "move") {
            dragging.current = false;
            onEndMoveNode?.(id, position.current);
            document.removeEventListener("mousemove", onMouseMove);
            document.removeEventListener("mouseup", onMouseUp);
        }
    };

    return (
        <div
            ref={ nodeRef }
            className={ clsx(styles.stateNodeWrapper, currentTab?.activeControl === "trashcan" && styles.deleteMode) }
            style={ {
                cursor: getCursorStyle(),
                zIndex: getZIndex(),
            } }
            data-node-id={ id }
            onClick={
                currentTab?.activeControl === "cursor"
                    ? () => {
                        if (currentTab) {
                            updateTab({ ...currentTab, selectedState: id });
                        }
                    }
                    : undefined
            }>
            {isInitial && <ChevronsRight className={ styles.stateInitial }
            />}
            <div
                className={ clsx(
                    styles.stateNode,
                    isFinal && styles.stateFinal,
                    currentTab?.activeControl === "trashcan" && styles.deleteMode,
                    currentTab?.selectedState === id && styles.selected,
                ) }
                onMouseDown={ onMouseDown }
            >
                {label}
            </div>
        </div>
    );
}