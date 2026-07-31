import { createContext, useContext, useState, ReactNode } from "react";
import { StateModel } from "@/interface/Automaton.ts";


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
    nodes: StateModel[];
    setNodes: (nodes: StateModel[] | ((prev: StateModel[]) => StateModel[])) => void;
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
    const [ nodes, setNodes ] = useState<StateModel[]>([]);
    const [ edges, setEdges ] = useState<EdgeState[]>([]);
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