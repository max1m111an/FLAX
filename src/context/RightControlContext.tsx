import { createContext, useContext, useState, ReactNode } from "react";


interface ControlContextProps {
    activeControl: string | null;
    changeActiveControl: (value: string) => void;
}

const RightControlContext = createContext<ControlContextProps | undefined>(undefined);

export const RightControlProvider = ({ children }: { children: ReactNode }) => {
    const [ activeControl, setActiveControl ] = useState<string | null>(null);

    const changeActiveControl = (widget: string) => {
        if (activeControl !== widget) {
            setActiveControl(widget);
        } else {
            setActiveControl(null);
        }
    };

    return (
        <RightControlContext.Provider value={ { activeControl, changeActiveControl } }>
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