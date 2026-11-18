import { StateModel, TransitionModel } from "@/types/Automaton.ts";
import { NODE_RADIUS } from "@/configs/CanvasConst.ts";

const TANGENT_DEG = 5;

export default function calculatePoints(
    edge: TransitionModel,
    nodes: StateModel[],
    bend: number = 0,
): {
    x1: number;
    y1: number;
    x2: number;
    y2: number;
    cx: number;
    cy: number;
    curved: boolean;
    textX: number;
    textY: number;
    angle: number;
} | null {
    const startNode = nodes.find((node) => node.id === edge.from);
    const endNode = nodes.find((node) => node.id === edge.to);

    if (!startNode || !endNode) return null;

    const startCenter = {
        x: startNode.x + NODE_RADIUS,
        y: startNode.y + NODE_RADIUS,
    };

    const endCenter = {
        x: endNode.x + NODE_RADIUS,
        y: endNode.y + NODE_RADIUS,
    };

    const dx = endCenter.x - startCenter.x;
    const dy = endCenter.y - startCenter.y;

    const len = Math.sqrt(dx * dx + dy * dy) || 1;
    const r = NODE_RADIUS;

    let x1 = startCenter.x + (dx / len) * r;
    let y1 = startCenter.y + (dy / len) * r;
    let x2 = endCenter.x - (dx / len) * r;
    let y2 = endCenter.y - (dy / len) * r;

    const normalX = -dy / len;
    const normalY = dx / len;

    const curved = bend !== 0;

    if (curved) {
        const sgn = bend >= 0 ? 1 : -1;
        const rad = (TANGENT_DEG * Math.PI) / 180;

        const a1 = rad * sgn;
        const c1 = Math.cos(a1);
        const s1 = Math.sin(a1);
        const sux = dx / len;
        const suy = dy / len;
        const rx1 = sux * c1 - suy * s1;
        const ry1 = sux * s1 + suy * c1;
        x1 = startCenter.x + rx1 * r;
        y1 = startCenter.y + ry1 * r;

        const a2 = -rad * sgn;
        const c2 = Math.cos(a2);
        const s2 = Math.sin(a2);
        const eux = -dx / len;
        const euy = -dy / len;
        const rx2 = eux * c2 - euy * s2;
        const ry2 = eux * s2 + euy * c2;
        x2 = endCenter.x + rx2 * r;
        y2 = endCenter.y + ry2 * r;
    }

    const midX = (x1 + x2) / 2;
    const midY = (y1 + y2) / 2;

    const cx = midX + normalX * bend;
    const cy = midY + normalY * bend;

    let textX: number;
    let textY: number;

    if (curved) {
        const apexX = 0.25 * x1 + 0.5 * cx + 0.25 * x2;
        const apexY = 0.25 * y1 + 0.5 * cy + 0.25 * y2;

        const chordDx = x2 - x1;
        const chordDy = y2 - y1;
        const chordLen = Math.sqrt(chordDx * chordDx + chordDy * chordDy) || 1;
        const chordNX = -chordDy / chordLen;
        const chordNY = chordDx / chordLen;

        const bulge = (apexX - midX) * chordNX + (apexY - midY) * chordNY;
        const side = bulge >= 0 ? 1 : -1;

        const goingDown = chordNY * side >= 0;
        const offset = goingDown ? 24 : 12;
        textX = apexX + chordNX * side * offset;
        textY = apexY + chordNY * side * offset;
    } else {
        let textNormalX = normalX;
        let textNormalY = normalY;

        if (textNormalY >= 0) {
            textNormalX *= -1;
            textNormalY *= -1;
        }

        textX = midX + textNormalX * 12;
        textY = midY + textNormalY * 12;
    }

    let angle = Math.atan2(dy, dx) * (180 / Math.PI);

    if (angle > 90 || angle < -90) {
        angle += 180;
    }

    return {
        x1,
        y1,
        x2,
        y2,
        cx,
        cy,
        curved,
        textX,
        textY,
        angle,
    };
}

export const buildSelfLoop = (stateX: number, stateY: number) => {
    const cx = stateX + NODE_RADIUS;
    const cy = stateY + NODE_RADIUS;
    const r = NODE_RADIUS;

    const attachX = 18;
    const attachY = cy - Math.sqrt(r * r - attachX * attachX);

    const startX = cx - attachX;
    const endX = cx + attachX;

    const c1x = startX + 2;
    const c2x = endX - 2;
    const ctrlY = attachY - 96;

    const d = `M ${startX} ${attachY} C ${c1x} ${ctrlY}, ${c2x} ${ctrlY}, ${endX} ${attachY}`;

    const textX = cx;
    const textY = attachY - 84;

    return { d, textX, textY };
};
