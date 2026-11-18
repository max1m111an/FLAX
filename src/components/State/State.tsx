import React, { useLayoutEffect, useRef } from "react";
import ChevronsRight from "@/assets/svg/ChevronsRight.svg?react";
import clsx from "clsx";
import styles from "./StateNode.module.scss";
import { EDGE_ATTACH_TOLERANCE, NODE_RADIUS, NODE_SIZE } from "@/configs/CanvasConst.ts";
import { ActiveControl } from "@/types/Tabs.ts";

interface StateProps {
    id: number;
    label: string;
    initialPosition?: { x: number; y: number };
    isInitial?: boolean;
    isFinal?: boolean;

    activeControl: ActiveControl;
    highlightStatus?: "success" | "error" | null;

    onDeleteNode?: (id: number) => void;

    onEndMoveNode?: (id: number, pos: { x: number; y: number }) => void;

    onStartEdge?: (pos: { x: number; y: number }) => void;
    onMoveEdge?: (pos: { x: number; y: number }) => void;
    onEndEdge?: (hoveredNodeId: number | undefined) => void;
    onMoveNode?: (id: number, pos: { x: number; y: number }) => void;

    getCanvasRect: () => DOMRect;
    onContextMenu?: (e: React.MouseEvent<HTMLDivElement>) => void;
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
        getCanvasRect,
        onContextMenu,
        activeControl,
        highlightStatus,
    }: StateProps) {
    const nodeRef = useRef<HTMLDivElement | null>(null);
    const position = useRef(initialPosition);
    const offset = useRef({ x: 0, y: 0 });
    const dragging = useRef(false);


    const { x: initialX, y: initialY } = initialPosition;

    useLayoutEffect(() => {
        if (!dragging.current && nodeRef.current) {
            position.current = { x: initialX, y: initialY };
            nodeRef.current.style.left = `${initialX}px`;
            nodeRef.current.style.top = `${initialY}px`;
        }
    }, [ initialX, initialY ]);


    const getCursorStyle = () => {
        if (activeControl === "move") {
            return dragging ? "grabbing" : "grab";
        }
        return "default";
    };

    const getZIndex = () => {
        return dragging ? 900 : "auto";
    };
    const onMouseDown = (e: React.MouseEvent) => {
        if (e.button !== 0) return;
        e.stopPropagation();
        e.preventDefault();
        if (activeControl === "move") {
            dragging.current = true;

            const canvasRect = getCanvasRect();
            offset.current = {
                x: (e.clientX - canvasRect.left) - position.current.x,
                y: (e.clientY - canvasRect.top) - position.current.y,
            };

            document.addEventListener("mousemove", onMouseMove);
            document.addEventListener("mouseup", onMouseUp);
            return;
        }
        if (activeControl === "edge") {
            document.addEventListener("mousemove", onMouseMove);
            document.addEventListener("mouseup", onMouseUp);
            return;
        }
        if (activeControl === "trashcan") {
            onDeleteNode?.(id);
            return;
        }

    };
    const onMouseMove = (e: MouseEvent) => {
        e.stopPropagation();
        e.preventDefault();
        if (activeControl === "edge") {
            if (!nodeRef.current) return;

            const canvasRect = getCanvasRect();
            const nodeRect = nodeRef.current.getBoundingClientRect();

            const cx = nodeRect.left + nodeRect.width / 2 - canvasRect.left;
            const cy = nodeRect.top + nodeRect.height / 2 - canvasRect.top;

            const point = {
                x: e.clientX - canvasRect.left,
                y: e.clientY - canvasRect.top,
            };

            const distToCenter = Math.hypot(point.x - cx, point.y - cy);

            if (distToCenter < NODE_RADIUS + EDGE_ATTACH_TOLERANCE) {
                return;
            }

            const dx = point.x - cx;
            const dy = point.y - cy;

            const len = Math.hypot(dx, dy) || 1;

            const start = {
                x: cx + (dx / len) * NODE_RADIUS,
                y: cy + (dy / len) * NODE_RADIUS,
            };

            onStartEdge?.(start);
            onMoveEdge?.(point);
        }
        if (activeControl === "move") {
            if (!dragging.current) return;

            const canvasRect = getCanvasRect();
            let newX = (e.clientX - canvasRect.left) - offset.current.x;
            let newY = (e.clientY - canvasRect.top) - offset.current.y;

            newX = Math.max(0, Math.min(newX, canvasRect.width - NODE_SIZE));
            newY = Math.max(0, Math.min(newY, canvasRect.height - NODE_SIZE));

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
        if (activeControl === "edge") {
            const element = document.elementFromPoint(e.clientX, e.clientY);
            const hoveredNode = element?.closest("[data-node-id]");
            const hoveredId = hoveredNode ? Number(hoveredNode.getAttribute("data-node-id")) : undefined;
            onEndEdge?.(hoveredId);

            document.removeEventListener("mousemove", onMouseMove);
            document.removeEventListener("mouseup", onMouseUp);
        }
        if (activeControl === "move") {
            dragging.current = false;
            onEndMoveNode?.(id, position.current);
            document.removeEventListener("mousemove", onMouseMove);
            document.removeEventListener("mouseup", onMouseUp);
        }
    };

    return (
        <div
            ref={ nodeRef }
            className={ clsx(styles.stateNodeWrapper, activeControl === "trashcan" && styles.deleteMode) }
            style={ {
                cursor: getCursorStyle(),
                zIndex: getZIndex(),
            } }
            data-node-id={ id }
        >
            {isInitial && <ChevronsRight className={ styles.stateInitial }
            />}
            <div
                className={ clsx(
                    styles.stateNode,
                    isFinal && styles.stateFinal,
                    activeControl === "trashcan" && styles.deleteMode,
                    highlightStatus === "success" && styles.traceSuccess,
                    highlightStatus === "error" && styles.traceError,
                ) }
                onMouseDown={ onMouseDown }
                onContextMenu={ onContextMenu }
            >
                {label}
            </div>
        </div>
    );
}