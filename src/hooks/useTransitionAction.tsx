import { useCurrentTab, useTabs } from "@/context/TabsContext.tsx";
import { useSnackbar } from "@/context/SnackbarContext.tsx";
import { modelServiceRegistry } from "@/services/ModelServiceRegistry.ts";
import { AppTab } from "@/types/Tabs.ts";
import { AutomatonModel } from "@/types/Automaton.ts";

export const useTransitionAction = () => {
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();
    const { showSnackbar } = useSnackbar();

    if (!currentTab || currentTab.type === "SETTINGS") {
        return {
            removeTransition: async () => {},
            addTransition: async () => {},
        };
    }

    const service = modelServiceRegistry.getService(currentTab.type);

    if (!service) {
        showSnackbar("error",`API сервис для автомата типа ${currentTab.type} не найден`);
        return {
            removeTransition: async () => {},
            addTransition: async () => {},
        };
    }
    const addTransition = async (startState: number, endState: number, symbolStr: string) => {
        if (!currentTab || currentTab.type === "SETTINGS") return;

        try {
            const response = await service.addTransition(currentTab.id, startState, endState, symbolStr);
            const newTabData: AppTab = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    transitions: [ ...currentTab.automaton.transitions, ...response.transition ],
                },
            };
            updateTab(newTabData);
        } catch (error) {
            console.error("Ошибка при добавлении перехода:", error);
        }
    };

    const removeTransition = async (id: number) => {
        const targetTransition = currentTab.automaton.transitions.find((t) => t.id === id);
        if (!targetTransition) return;

        const transitionsToDelete = currentTab.automaton.transitions.filter(
            (t) => t.from === targetTransition.from && t.to === targetTransition.to,
        );

        try {
            await Promise.all(
                transitionsToDelete.map((t) =>
                    service.removeTransition(currentTab.id, t.id),
                ),
            );

            const idsToDelete = transitionsToDelete.map((t) => t.id);
            const newTabData = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    transitions: currentTab.automaton.transitions.filter(
                        (t) => !idsToDelete.includes(t.id),
                    ),
                } as AutomatonModel,
            };

            updateTab(newTabData);
        } catch (error) {
            console.error("Ошибка при удалении ребра:", error);
        }
    };
    return { addTransition, removeTransition };
};