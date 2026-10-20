import { createContext, useContext, useState, ReactNode, ComponentType, SVGProps } from "react";
import { useLocation, useNavigate, useParams } from "react-router-dom";
import { ROUTES } from "@/configs/RoutesConst.ts";
import { save } from "@tauri-apps/plugin-dialog";
import { basename } from "@tauri-apps/api/path";
import { useRecentFiles } from "@/context/RecentFilesContext.tsx";
import { ConfirmCloseModal } from "@/components/ConfirmCloseModal/ConfirmCloseModal.tsx";
import { model } from "@/types/Automaton.ts";
import { AppTab } from "@/types/Tabs.ts";
import { modelServiceRegistry } from "@/services/ModelServiceRegistry.ts";
import { buildTab } from "@/utils/tabFactory.ts";
import { models } from "@/data/Models.ts";
import { jffService } from "@/services/jffService.ts";


interface TabsContextProps {
    tabs: AppTab[];
    addTab: (model: model, type?: string) => Promise<AppTab | void>;
    addUtilTab: (type: string, icon: ComponentType<SVGProps<SVGSVGElement>>) => Promise<AppTab | void>;
    removeTab: (tab: AppTab) => void;
    updateTab: (updatedTab: AppTab) => void;
    loadTab: (path: string) => void;
    saveTab: (currentTab: AppTab) => Promise<boolean>;
}

const TabsContext = createContext<TabsContextProps | undefined>(undefined);

export const TabsProvider = ({ children }: { children: ReactNode }) => {
    const [ tabs, setTabs ] = useState<AppTab[]>([]);
    const [ pendingCloseTab, setPendingCloseTab ] = useState<AppTab | null>(null);
    const navigate = useNavigate();
    const location = useLocation();
    const { addFile } = useRecentFiles();

    const addTab = async (model: model): Promise<AppTab | void> => {
        try {
            const service = modelServiceRegistry.getService(model.type);

            const response = await service.createAutomaton("Без названия*");

            const newTab = buildTab(response.automaton, model.icon, model.type);

            setTabs([ ...tabs, newTab ]);

            navigate(`/models/${newTab.id}`);

            return newTab;

        } catch (error) {
            console.error("Ошибка при создании автомата:", error);
        }
    };

    const addUtilTab = async (type: string, icon: ComponentType<SVGProps<SVGSVGElement>>): Promise<AppTab | void> => {
        if (type === "SETTINGS") {
            if (tabs.some((t) => t.type === "SETTINGS")) {
                navigate(ROUTES.SETTINGS);
                return;
            }
            const settingsTab: AppTab = { id: 0, title: "Настройки", type: "SETTINGS", isSaved: true, savedPath: null, icon };
            setTabs([ ...tabs, settingsTab ]);
            navigate(ROUTES.SETTINGS);
            return settingsTab;
        }
    };

    const loadTab = async (path: string) => { //todo hardcode, incorrect API loadJFF
        const response = await jffService.loadJff({ path });
        const currentModel = models.find((model) => model.type === "FA");

        if (!response.automaton) {
            throw new Error("Ошибка загрузки: бэкенд не вернул данные автомата");
        }
        if (!currentModel) {
            throw new Error("Ошибка загрузки: бэкенд не вернул данные автомата");
        }

        const newTab = buildTab(response.automaton, currentModel?.icon, "FA", true, path);

        setTabs([ ...tabs, newTab ]);
        navigate(`/models/${newTab.id}`);
    };

    const updateTab = (updatedTab: AppTab) => {
        setTabs((prev) => prev.map((t) => t.id === updatedTab.id ? updatedTab : t));
    };

    const saveTab = async (currentTab: AppTab): Promise<boolean> => {
        if (currentTab.type === "SETTINGS") return false;

        const service = modelServiceRegistry.getService(currentTab.type);
        try {
            const filePath = await save({
                defaultPath: `${currentTab?.title}.jff`,
                filters: [ { name: "Единый формат .jff", extensions: [ "jff" ] } ],
            });

            if (!filePath || currentTab === undefined) return false;

            const fileName = await basename(filePath);
            const nameWithoutExt = fileName.replace(/\.jff$/i, "");

            await service.saveAutomaton(currentTab.id, filePath);

            updateTab({
                ...currentTab,
                title: nameWithoutExt,
                isSaved: true,
                savedPath: filePath,
            });

            addFile(filePath);

            return true;

        } catch (error) {
            console.error("Ошибка при сохранении файла:", error);
            return false;
        }
    };
    const closeTab = async (self_tab: AppTab): Promise<void> => {
        if (self_tab.type === "SETTINGS") {
            removeTabFromState(self_tab);
            return;
        }

        const service = modelServiceRegistry.getService(self_tab.type);

        await service.deleteAutomaton(self_tab.id);

        removeTabFromState(self_tab);
    };

    const removeTabFromState = (self_tab: AppTab) => {
        const newTabs = tabs.filter((tab) => tab.id !== self_tab.id);
        setTabs(newTabs);

        const tabPath = self_tab.type === "SETTINGS" ? ROUTES.SETTINGS : `/models/${self_tab.id}`;
        if (location.pathname === tabPath) {
            if (newTabs.length > 0) {
                const lastTab = newTabs[newTabs.length - 1];
                const navigatePath = lastTab.type === "SETTINGS" ? ROUTES.SETTINGS : `/models/${lastTab.id}`;
                navigate(navigatePath);
            } else {
                navigate(ROUTES.MAIN);
            }
        }
    };

    const removeTab = (self_tab: AppTab): void => {
        if (!self_tab.isSaved) {
            setPendingCloseTab(self_tab);
            return;
        }
        closeTab(self_tab);
    };

    const handleConfirmSave = async (): Promise<void> => {
        if (!pendingCloseTab) return;
        const saved = await saveTab(pendingCloseTab);
        if (saved) {
            await closeTab(pendingCloseTab);
        }
        setPendingCloseTab(null);
    };

    const handleConfirmDiscard = (): void => {
        if (!pendingCloseTab) return;
        closeTab(pendingCloseTab);
        setPendingCloseTab(null);
    };

    const handleCancelClose = (): void => {
        setPendingCloseTab(null);
    };

    return (
        <TabsContext.Provider value={ { tabs, addTab, removeTab, updateTab, loadTab, addUtilTab, saveTab } }>
            {children}
            {pendingCloseTab && (
                <ConfirmCloseModal
                    title="Несохранённые изменения"
                    message="Файл не сохранён. Сохранить перед закрытием?"
                    onSave={ handleConfirmSave }
                    onDiscard={ handleConfirmDiscard }
                    onCancel={ handleCancelClose }
                />
            )}
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
export const useCurrentTab = (): AppTab | undefined => {
    const { tabs } = useTabs();
    const { id } = useParams();
    return tabs.find((tab) => String(tab.id) === id);
};