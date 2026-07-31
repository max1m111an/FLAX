import { createContext, useContext, useState, ReactNode } from "react";


export interface NodeState {
    id: number;
    x: number;
    y: number;
    name: string;
    isFinal: boolean;
    isInitial: boolean;
}
export interface EdgeState {
    id: number;
    idStartNode: number;
    idEndNode: number;
    state?: string[];
    isEditing?: boolean;
}

interface ControlContextProps {
    activeControl: string | null;
    activePane: string | null;
    changePane: (value: string) => void;
    changeControl: (value: string) => void;
    nodes: NodeState[];
    setNodes: (nodes: NodeState[] | ((prev: NodeState[]) => NodeState[])) => void;
    edges: EdgeState[];
    setEdges: (edges: EdgeState[] | ((prev: EdgeState[]) => EdgeState[])) => void;
    selectedNode: number | null;
    setSelectedNode: (id: number | null) => void;
    selectedEdge: number | null;
    setSelectedEdge: (id: number | null) => void;

}

const ControlContext = createContext<ControlContextProps | undefined>(undefined);

export const ControlProvider = ({ children }: { children: ReactNode }) => {
    const [ activePane, setActivePane ] = useState<string | null>(null);
    const [ activeControl, setActiveControl ] = useState<string | null>("cursor");
    const [ nodes, setNodes ] = useState<NodeState[]>([
        { id: 0, x: 100, y: 100, name: "q0", isFinal: false, isInitial: true },
        { id: 1, x: 500, y: 100, name: "q1", isFinal: true, isInitial: false },
        { id: 2, x: 100, y: 500, name: "q2", isFinal: false, isInitial: false },
        { id: 3, x: 500, y: 500, name: "q3", isFinal: false, isInitial: false },
    ]);
    const [ edges, setEdges ] = useState<EdgeState[]>([
        { id: 0, idStartNode: 2, idEndNode: 3, state: [ "1", "2", "3" ] },
        { id: 1, idStartNode: 1, idEndNode: 0, state: [ "λ" ] },
    ]);
    const [ selectedNode, setSelectedNode ] = useState<number | null>(null);
    const [ selectedEdge, setSelectedEdge ] = useState<number | null>(null);

    const changePane = (widget: string) => {
        if (activePane !== widget) {
            setActivePane(widget);
        } else {
            setActivePane(null);
        }
    };

    const changeControl = (widget: string) => {
        setActiveControl(widget);
    };

    return (
        <ControlContext.Provider value={ {
            activeControl,
            activePane,
            changePane,
            changeControl,
            nodes,
            setNodes,
            edges,
            setEdges,
            selectedNode,
            setSelectedNode,
            selectedEdge,
            setSelectedEdge,
        } }>
            {children}
        </ControlContext.Provider>
    );
};
// eslint-disable-next-line react-refresh/only-export-components
export const useControl = () => {
    const context = useContext(ControlContext);
    if (!context) throw new Error("useTabs must be used within TabsProvider");
    return context;
};