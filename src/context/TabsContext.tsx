import { createContext, useContext, useState, ReactNode } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import { ROUTES } from "@/configs/RoutesConst.ts";
import { model } from "@/data/models.ts";

export interface tab{
    id: number;
    title: string;
    model: model;
}

interface TabsContextProps {
    tabs: tab[];
    addTab: (model: model) => void;
    removeTab: (tab: tab) => void;
}

const TabsContext = createContext<TabsContextProps | undefined>(undefined);

export const TabsProvider = ({ children }: { children: ReactNode }) => {
    const [ tabs, setTabs ] = useState<tab[]>([]);
    const navigate = useNavigate();

    const addTab = (model:model) : void => {
        const id: number = Date.now();
        const newTab: tab = {
            id: id,
            title: "Без названия*",
            model: model,
        };
        setTabs([ ...tabs, newTab ]);
        navigate(`/models/${id}`);
    };
    const location = useLocation();

    const removeTab = (self_tab: tab): void => {
        const newTabs = tabs.filter((tab) => tab.id !== self_tab.id);

        const isCurrent = location.pathname === `/models/${self_tab.id}`;

        setTabs(newTabs);

        if (isCurrent) {
            if (newTabs.length > 0) {
                const lastTab = newTabs[newTabs.length - 1];
                navigate(`/models/${lastTab.id}`);
            } else {
                navigate(ROUTES.MAIN);
            }
        }
    };

    return (
        <TabsContext.Provider value={ { tabs, addTab, removeTab } }>
            {children}
        </TabsContext.Provider>
    );
};
// eslint-disable-next-line react-refresh/only-export-components
export const useTabs = () => {
    const context = useContext(TabsContext);
    if (!context) throw new Error("useTabs must be used within TabsProvider");
    return context;
};