import clsx from "clsx";
import styles from "./Edge.module.scss";
import { useCurrentTab } from "@/context/TabsContext.tsx";

interface EdgeProps {
    x1: number;
    y1: number;
    x2: number;
    y2: number;
    cx?: number;
    cy?: number;
    curved?: boolean;
    loopD?: string;
    textX?: number;
    textY?: number;
    angle?: number;
    label?: string;
    isEditing?: boolean;
    onDeleteEdge?: (id: number) => void;
    id?: number;
    from?: number;
    to?: number;
}

export default function Edge(
    {
        x1,
        y1,
        x2,
        y2,
        cx,
        cy,
        curved,
        loopD,
        label,
        isEditing,
        onDeleteEdge,
        id,
        textX,
        textY,
        angle,
        from,
        to,
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

    const relatedTransitionIds = (currentTab?.automaton.transitions ?? [])
        .filter((t) => from !== undefined && to !== undefined && t.from === from && t.to === to)
        .map((t) => t.id);

    const highlight = currentTab?.selectedTransition?.find((h) => relatedTransitionIds.includes(h.id));

    const traceStatus = highlight && highlight.status
        ? (highlight.status === "success" ? styles.traceSuccess : styles.traceError)
        : "";

    return (
        <g
            onMouseDown={ handleDelete }
            className={ clsx(
                styles.edge,
                currentTab?.activeControl === "trashcan" && styles.deleteMode,
                traceStatus,
            ) }
        >
            <defs>
                <marker id={ `arrowhead-${id}` } markerWidth="14" markerHeight="14" refX="12" refY="7" orient="auto">
                    <line x1="2" y1="3" x2="12" y2="7" />
                    <line x1="2" y1="11" x2="12" y2="7" />
                </marker>
            </defs>
            {loopD ? (
                <>
                    <path
                        d={ loopD }
                        className={ styles.delete }
                    />
                    <path
                        d={ loopD }
                        markerEnd={ `url(#arrowhead-${id})` }
                        className={ clsx(highlight && !highlight.status && styles.selected) }
                        style={ { cursor: currentTab?.activeControl === "trashcan" ? "pointer" : "default" } }
                    />
                </>
            ) : curved && cx !== undefined && cy !== undefined ? (
                <>
                    <path
                        d={ `M ${x1} ${y1} Q ${cx} ${cy} ${x2} ${y2}` }
                        className={ styles.delete }
                    />
                    <path
                        d={ `M ${x1} ${y1} Q ${cx} ${cy} ${x2} ${y2}` }
                        markerEnd={ `url(#arrowhead-${id})` }
                        className={ clsx(highlight && !highlight.status && styles.selected) }
                        style={ { cursor: currentTab?.activeControl === "trashcan" ? "pointer" : "default" } }
                    />
                </>
            ) : (
                <>
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
                        className={ clsx(highlight && !highlight.status && styles.selected) }
                        style={ { cursor: currentTab?.activeControl === "trashcan" ? "pointer" : "default" } }
                    />
                </>
            )}
            {!isEditing &&
                label &&
                textX !== undefined &&
                textY !== undefined &&
                angle !== undefined && (
                <text
                    x={ textX }
                    y={ textY }
                    textAnchor="middle"
                    className={ clsx(styles.edgeLabel, highlight && !highlight.status && styles.selected) }
                    transform={ `rotate(${angle}, ${textX}, ${textY})` }
                    style={ { cursor: currentTab?.activeControl === "trashcan" ? "pointer" : "default" } }
                >
                    {label}
                </text>
            )}
        </g>
    );
}