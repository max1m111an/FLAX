import { useEffect, useRef } from "react";
import ChevronsRight from "@/assets/svg/ChevronsRight.svg?react";

interface StateNodeProps {
    isFinal?: boolean;
    isInitial?: boolean;
    label: string;
    initialPosition?: { x: number; y: number };
}

export default function StateNode({
                                      isFinal = true,
                                      isInitial = true,
                                      label,
                                      initialPosition = { x: 0, y: 0 },
                                  }: StateNodeProps) {
    const nodeRef = useRef<HTMLDivElement | null>(null);
    const position = useRef(initialPosition);
    const offset = useRef({ x: 0, y: 0 });
    const dragging = useRef(false);

    useEffect(() => {
        if (nodeRef.current) {
            nodeRef.current.style.left = `${position.current.x}px`;
            nodeRef.current.style.top = `${position.current.y}px`;
        }
    }, []);

    const onMouseDown = (e: React.MouseEvent) => {
        e.stopPropagation();
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
    };

    const onMouseMove = (e: MouseEvent) => {
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
    };

    const onMouseUp = () => {
        dragging.current = false;
        document.removeEventListener("mousemove", onMouseMove);
        document.removeEventListener("mouseup", onMouseUp);
    };

    return (
        <div
            ref={nodeRef}
            className="state-node-wrapper"
            style={{
                position: 'fixed',
                left: 0,
                top: 0,
                cursor: dragging.current ? 'grabbing' : 'grab',
                zIndex: dragging.current ? 1000 : 'auto', // Поднимаем перетаскиваемый узел
            }}
        >
            {isInitial && <ChevronsRight className="state-initial" />}
            <div
                className={`state-node ${isFinal && "state-final"}`}
                onMouseDown={onMouseDown}
            >
                {label}
            </div>
        </div>
    );
}