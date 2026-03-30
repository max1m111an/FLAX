import { Edge, NodeState } from "@/widgets/ModelCanvasWidget.tsx";

export default function calculatePoints(
    edge: Edge,
    nodes: NodeState[],
): {
    x1: number;
    y1: number;
    x2: number;
    y2: number;
    textX: number;
    textY: number;
    angle: number;
} | null {
    const startNode = nodes.find((node) => node.id === edge.idStartNode);
    const endNode = nodes.find((node) => node.id === edge.idEndNode);

    if (!startNode || !endNode) return null;

    const startCenter = {
        x: startNode.x + 32,
        y: startNode.y + 32,
    };

    const endCenter = {
        x: endNode.x + 32,
        y: endNode.y + 32,
    };

    const dx = endCenter.x - startCenter.x;
    const dy = endCenter.y - startCenter.y;

    const len = Math.sqrt(dx * dx + dy * dy) || 1;
    const r = 32;

    const x1 = startCenter.x + (dx / len) * r;
    const y1 = startCenter.y + (dy / len) * r;
    const x2 = endCenter.x - (dx / len) * r;
    const y2 = endCenter.y - (dy / len) * r;

    const midX = (x1 + x2) / 2;
    const midY = (y1 + y2) / 2;

    let normalX = -dy / len;
    let normalY = dx / len;

    if (normalY >= 0) {
        normalX *= -1;
        normalY *= -1;
    }

    const offset = 12;

    const textX = midX + normalX * offset;
    const textY = midY + normalY * offset;

    let angle = Math.atan2(dy, dx) * (180 / Math.PI);

    if (angle > 90 || angle < -90) {
        angle += 180;
    }

    return {
        x1,
        y1,
        x2,
        y2,
        textX,
        textY,
        angle,
    };
}