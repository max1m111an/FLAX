import clsx from "clsx";
import styles from "./Transition.module.scss";
import React, { useId } from "react";
import { ActiveControl } from "@/types/Tabs.ts";

interface EdgeProps {
    x1: number; y1: number; x2: number; y2: number;
    cx?: number; cy?: number; curved?: boolean; loopD?: string;
    textX?: number; textY?: number; angle?: number;
    label?: string; isEditing?: boolean;
    id?: number;
    onContextMenu?: (e: React.MouseEvent<SVGGElement>) => void;
    activeControl?: ActiveControl;
    highlightStatus?: "success" | "error" | null;
    isSelected?: boolean;
    onMouseDown?: (id: number) => void;
}

export const Transition = React.memo(function Transition(
    {
        x1, y1, x2, y2, cx, cy, curved, loopD, label, isEditing,
        id, textX, textY, angle, onContextMenu,
        activeControl, highlightStatus, isSelected, onMouseDown,
    }: EdgeProps) {

    const fallbackId = useId();
    const edgeId = id !== undefined ? id : fallbackId.replace(/:/g, "");
    const markerId = `arrowhead-${edgeId}`;

    const traceStatus = highlightStatus === "success" ? styles.traceSuccess : highlightStatus === "error" ? styles.traceError : "";
    const cursorStyle = activeControl === "trashcan" ? "pointer" : "default";

    return (
        <g
            onMouseDown={ (e) => {
                e.stopPropagation();
                if (id !== undefined && onMouseDown) {
                    onMouseDown(id);
                }
            } }
            className={ clsx(
                styles.edge,
                activeControl === "trashcan" && styles.deleteMode,
                traceStatus,
            ) }
            onContextMenu={ onContextMenu }
            style={ { pointerEvents: id !== undefined ? "all" : "none" } }
        >
            <defs>
                <marker id={ markerId } markerWidth="14" markerHeight="14" refX="12" refY="7" orient="auto">
                    <line x1="2" y1="3" x2="12" y2="7" />
                    <line x1="2" y1="11" x2="12" y2="7" />
                </marker>
            </defs>
            {loopD ? (
                <>
                    <path d={ loopD } className={ styles.delete } />
                    <path d={ loopD } markerEnd={ `url(#${markerId})` } className={ clsx(isSelected && styles.selected) } style={ { cursor: cursorStyle } } />
                </>
            ) : curved && cx !== undefined && cy !== undefined ? (
                <>
                    <path d={ `M ${x1} ${y1} Q ${cx} ${cy} ${x2} ${y2}` } className={ styles.delete } />
                    <path d={ `M ${x1} ${y1} Q ${cx} ${cy} ${x2} ${y2}` } markerEnd={ `url(#${markerId})` } className={ clsx(isSelected && styles.selected) } style={ { cursor: cursorStyle } } />
                </>
            ) : (
                <>
                    <line x1={ x1 } y1={ y1 } x2={ x2 } y2={ y2 } className={ styles.delete } />
                    <line x1={ x1 } y1={ y1 } x2={ x2 } y2={ y2 } markerEnd={ `url(#${markerId})` } className={ clsx(isSelected && styles.selected) } style={ { cursor: cursorStyle } } />
                </>
            )}
            {!isEditing && label && textX !== undefined && textY !== undefined && angle !== undefined && (
                <text
                    x={ textX } y={ textY } textAnchor="middle"
                    className={ clsx(styles.edgeLabel, isSelected && styles.selected) }
                    transform={ `rotate(${angle}, ${textX}, ${textY})` }
                    style={ { cursor: cursorStyle } }
                >
                    {label}
                </text>
            )}
        </g>
    );
});

export default Transition;