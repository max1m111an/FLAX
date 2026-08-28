import { createContext, useContext, useState, ReactNode } from "react";


interface ControlContextProps {
    selectedNode: number | null;
    setSelectedNode: (id: number | null) => void;
    selectedEdge: number | null;
    setSelectedEdge: (id: number | null) => void;

}

const ControlContext = createContext<ControlContextProps | undefined>(undefined);

export const ControlProvider = ({ children }: { children: ReactNode }) => {
    const [ selectedNode, setSelectedNode ] = useState<number | null>(null);
    const [ selectedEdge, setSelectedEdge ] = useState<number | null>(null);

    return (
        <ControlContext.Provider value={ {
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