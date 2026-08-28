import clsx from "clsx";
import styles from "./Edge.module.scss";
import { useCurrentTab } from "@/context/TabsContext.tsx";

interface EdgeProps {
    x1: number;
    x2: number;
    y1: number;
    y2: number;
    textX?: number;
    textY?: number;
    angle?: number;
    label?: string;
    isEditing?: boolean;
    onDeleteEdge?: (id: number) => void;
    id?: number;
}

export default function Edge(
    {
        x1,
        y1,
        x2,
        y2,
        label,
        isEditing,
        onDeleteEdge,
        id,
        textX,
        textY,
        angle,
    }: EdgeProps) {
    const currentTab = useCurrentTab();

    const handleDelete = (e: React.MouseEvent) => {
        e.stopPropagation();
        e.preventDefault();

        if (currentTab?.activeControl === "trashcan") {
            if (id !== undefined && onDeleteEdge) {
                onDeleteEdge(id);
            }
        }
    };

    return (
        <g
            onMouseDown={ handleDelete }
            className={ clsx(styles.edge, currentTab?.activeControl === "trashcan" && styles.deleteMode) }
        >
            <defs>
                <marker id={ `arrowhead-${id}` } markerWidth="14" markerHeight="14" refX="12" refY="7" orient="auto">
                    <line x1="2" y1="3" x2="12" y2="7" />
                    <line x1="2" y1="11" x2="12" y2="7" />
                </marker>
            </defs>
            <line
                x1={ x1 }
                y1={ y1 }
                x2={ x2 }
                y2={ y2 }
                className={ styles.delete }
            />
            <line
                x1={ x1 }
                y1={ y1 }
                x2={ x2 }
                y2={ y2 }
                markerEnd={ `url(#arrowhead-${id})` }
                className={ clsx(currentTab?.selectedTransition === id && styles.selected) }
                style={ { cursor: currentTab?.activeControl === "trashcan" ? "pointer" : "default" } }
            />
            {!isEditing &&
                label &&
                textX !== undefined &&
                textY !== undefined &&
                angle !== undefined && (
                <text
                    x={ textX }
                    y={ textY }
                    textAnchor="middle"
                    className={ clsx(styles.edgeLabel, currentTab?.selectedTransition === id && styles.selected) }
                    transform={ `rotate(${angle}, ${textX}, ${textY})` }
                    style={ { cursor: currentTab?.activeControl === "trashcan" ? "pointer" : "default" } }
                >
                    {label}
                </text>
            )}
        </g>
    );
}