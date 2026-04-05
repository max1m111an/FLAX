import StateNode from "@/components/StateNode.tsx";
import {useState} from "react";
import {useControl} from "@/context/RightControlContext.tsx";

interface Node {
    id: number;
    x: number;
    y: number;
    name: string;
    isFinal: boolean;
    isInitial: boolean;
}
export default function ModelCanvasWidget() {
    const { activeControl } = useControl();

    const [nodes, setNodes] = useState<Node[]>([
            { id:0, x: 100, y: 100, name: "q0", isFinal: false, isInitial: true },
            { id:1, x: 100, y: 200, name: "q1", isFinal: true, isInitial: false },
            { id:2, x: 100, y: 300, name: "q2", isFinal: false, isInitial: false },
        ]
    );
    const addNode = (e: React.MouseEvent<HTMLDivElement>) => {
        const x = e.clientX - 32;
        const y = e.clientY - 32;
        const nextIndex = nodes.length;
        const newNode: Node = {
            id: Date.now(),
            x: x,
            y: y,
            name: `q${nextIndex}`,
            isInitial: false,
            isFinal: false,
        }
        setNodes([...nodes, newNode]);
    }
    return (
        <div
            className="model-canvas-wrapper"
            onClick={activeControl === "node" ? addNode : undefined}
        >
            {nodes.map(node => (
                <StateNode
                    label={node.name}
                    initialPosition={{ x: node.x, y: node.y }}
                    isInitial={node.isInitial}
                    isFinal={node.isFinal}
                />
            ))}
        </div>
    );
}