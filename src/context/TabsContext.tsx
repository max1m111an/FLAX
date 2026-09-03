import { createContext, useContext, useState, ReactNode } from "react";
import { useLocation, useNavigate, useParams } from "react-router-dom";
import { ROUTES } from "@/configs/RoutesConst.ts";
import { model } from "@/data/models.ts";
import { createNewNFA } from "@/api/nfaAPI.ts";
import { AutomatonModel } from "@/types/Automaton.ts";
import type { Trace } from "@/api/nfaAPI.ts";

export interface TraceHighlight {
    id: number;
    status?: "success" | "error";
}

export interface tab{
    id: number;
    title: string;
    model: model;
    automaton: AutomatonModel;
    activeControl: string | null;
    activePanel: string | null;
    selectedState: TraceHighlight[] | null;
    selectedTransition: TraceHighlight[] | null;
    selectedNodeId: number | null;
    testMode: string;
    testInput: string;
    pendingTestLine: string | null;
    pendingTraces: Trace[] | null;

}

interface TabsContextProps {
    tabs: tab[];
    addTab: (model: model, type?: string) => Promise<tab | void>;
    removeTab: (tab: tab) => void;
    updateTab: (updatedTab: tab) => void;
    loadTab: (automaton: AutomatonModel, model: model) => void;
}

const TabsContext = createContext<TabsContextProps | undefined>(undefined);

export const TabsProvider = ({ children }: { children: ReactNode }) => {
    const [ tabs, setTabs ] = useState<tab[]>([]);
    const navigate = useNavigate();

    const addTab = async (model: model, type: string = "Без названия*"): Promise<tab | void> => {
        if (type === "Настройки") {
            const existingSettingsTab = tabs.find((t) => t.title === "Настройки");
            if (existingSettingsTab) {
                navigate(ROUTES.SETTINGS);
                return;
            }
        }
        const response = await createNewNFA("Без названия*");
        if (response.status == 200) {
            const newTab: tab = {
                id: response.automaton.id,
                title: response.automaton.name,
                model,
                automaton: response.automaton,
                activeControl: "cursor",
                activePanel: null,
                selectedState: null,
                selectedTransition: null,
                selectedNodeId: null,
                testMode: "solo",
                testInput: "",
                pendingTestLine: null,
                pendingTraces: null,
            };
            setTabs([ ...tabs, newTab ]);

            if (type == "Настройки") {
                navigate(ROUTES.SETTINGS);
            } else if (type == "Без названия*") {
                navigate(`/models/${newTab.id}`);
            }
            return newTab;
        }
    };

    const location = useLocation();

    const loadTab = (automaton: AutomatonModel, model: model) => {
        const newTab: tab = {
            id: automaton.id,
            title: automaton.name,
            model,
            automaton: automaton,
            activeControl: "cursor",
            activePanel: null,
            selectedState: null,
            selectedTransition: null,
            selectedNodeId: null,
            testMode: "solo",
            testInput: "",
            pendingTestLine: null,
            pendingTraces: null,
        };
        setTabs([ ...tabs, newTab ]);
        navigate(`/models/${newTab.id}`);
    };

    const updateTab = (updatedTab: tab) => {
        setTabs((prev) => prev.map((t) => t.id === updatedTab.id ? updatedTab : t));
    };
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
        <TabsContext.Provider value={ { tabs, addTab, removeTab, updateTab, loadTab } }>
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

// eslint-disable-next-line react-refresh/only-export-components
export const useCurrentTab = (): tab | undefined => {
    const { tabs } = useTabs();
    const { id } = useParams();
    return tabs.find((tab) => String(tab.id) === id);
};