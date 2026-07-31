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
    addTab: (model: model, type?: string) => void;
    removeTab: (tab: tab) => void;
}

const TabsContext = createContext<TabsContextProps | undefined>(undefined);

export const TabsProvider = ({ children }: { children: ReactNode }) => {
    const [ tabs, setTabs ] = useState<tab[]>([]);
    const navigate = useNavigate();

    const addTab = (model:model, type : string = "Без названия*" ) : void => {
        if (type === "Настройки") {
            const existingSettingsTab = tabs.find((t) => t.title === "Настройки");
            if (existingSettingsTab) {
                navigate(ROUTES.SETTINGS);
                return;
            }
        }

        const id: number = Date.now();
        const newTab: tab = {
            id: id,
            title: type,
            model: model,
        };
        setTabs([ ...tabs, newTab ]);
        if (type == "Без названия*") {
            navigate(`/models/${id}`);
        } else if (type == "Настройки") {
            navigate(ROUTES.SETTINGS);
        }
    };
    const location = useLocation();

    const removeTab = (self_tab: tab): void => {
        const newTabs = tabs.filter((tab) => tab.id !== self_tab.id);

        const tabPath = self_tab.title === "Настройки" ? ROUTES.SETTINGS : `/models/${self_tab.id}`;
        const isCurrent = location.pathname === tabPath;

        setTabs(newTabs);

        if (isCurrent) {
            if (newTabs.length > 0) {
                const lastTab = newTabs[newTabs.length - 1];
                const navigatePath = lastTab.title === "Настройки" ? ROUTES.SETTINGS : `/models/${lastTab.id}`;
                navigate(navigatePath);
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