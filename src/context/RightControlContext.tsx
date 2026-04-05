import { createContext, useContext, useState, ReactNode } from "react";


interface ControlContextProps {
    activeControl: string | null;
    activePane: string | null;
    changePane: (value: string) => void;
    changeControl: (value: string) => void;
}

const RightControlContext = createContext<ControlContextProps | undefined>(undefined);

export const RightControlProvider = ({ children }: { children: ReactNode }) => {
    const [ activePane, setActivePane ] = useState<string | null>(null);
    const [ activeControl, setActiveControl ] = useState<string | null>("cursor");

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
        <RightControlContext.Provider value={ {
            activeControl: activeControl,
            activePane: activePane,
            changePane: changePane,
            changeControl: changeControl
        } }>
            {children}
        </RightControlContext.Provider>
    );
};
// eslint-disable-next-line react-refresh/only-export-components
export const useControl = () => {
    const context = useContext(RightControlContext);
    if (!context) throw new Error("useTabs must be used within TabsProvider");
    return context;
};