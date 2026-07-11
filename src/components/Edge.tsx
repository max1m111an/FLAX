import { useControl } from "@/context/ControlContext.tsx";

interface EdgeProps {
    x1: number;
    x2: number;
    y1: number;
    y2: number;
    textX?: number;
    textY?: number;
    angle?: number;
    label?: string[];
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
    const { activeControl, selectedEdge } = useControl();

    const handleDelete = (e: React.MouseEvent) => {
        e.stopPropagation();
        e.preventDefault();

        if (activeControl === "trashcan") {
            if (id !== undefined && onDeleteEdge) {
                onDeleteEdge(id);
            }
        }
    };

    return (
        <g
            onMouseDown={ handleDelete }
            className={ activeControl === "trashcan" ? "delete-mode" : "" }
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
                className={ activeControl === "trashcan" ? "delete" : "" }
            />
            <line
                x1={ x1 }
                y1={ y1 }
                x2={ x2 }
                y2={ y2 }
                markerEnd={ `url(#arrowhead-${id})` }
                className={ `${selectedEdge == id && "selected"}` }
                style={ { cursor: activeControl === "trashcan" ? "pointer" : "default" } }
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
                    className={ `edge-label ${selectedEdge == id && "selected"}` }
                    transform={ `rotate(${angle}, ${textX}, ${textY})` }
                    style={ { cursor: activeControl === "trashcan" ? "pointer" : "default" } }
                >
                    {label.join(", ")}
                </text>
            )}
        </g>
    );
}