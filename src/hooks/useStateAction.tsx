import { useCurrentTab, useTabs } from "@/context/TabsContext.tsx";
import { useSnackbar } from "@/context/SnackbarContext.tsx";
import { AppTab } from "@/types/Tabs.ts";
import { StatePayload } from "@/types/Payloads.ts";
import { modelServiceRegistry } from "@/services/ModelServiceRegistry.ts";


export const useStateActions = () => {
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();
    const { showSnackbar } = useSnackbar();

    if (!currentTab || currentTab.type === "SETTINGS") {
        return {
            handleFinalChange: async () => {},
            handleNameChange: async () => {},
            handleXChange: async () => {},
            handleYChange: async () => {},
            handleInitialChange: async () => {},
            removeTransition: async () => {},
            addState: async () => {},
            removeState: async () => {},
            saveStatePosition: async () => {},
            moveStateLocal: async () => {},
        };
    }

    const service = modelServiceRegistry.getService(currentTab.type);
    if (!service) {
        showSnackbar("error",`API сервис для автомата типа ${currentTab.type} не найден`);
        return {
            handleFinalChange: async () => {},
            handleNameChange: async () => {},
            handleXChange: async () => {},
            handleYChange: async () => {},
            handleInitialChange: async () => {},
            removeTransition: async () => {},
            addState: async () => {},
            removeState: async () => {},
            saveStatePosition: async () => {},
            moveStateLocal: async () => {},

        };
    }

    const selectedStateId = currentTab?.selectedNodeId ?? null;
    const moveStateLocal = (id: number, pos: { x: number; y: number }) => {
        updateTab({
            ...currentTab,
            automaton: {
                ...currentTab.automaton,
                states: currentTab.automaton.states.map((state) =>
                    state.id === id ? { ...state, x: pos.x, y: pos.y } : state,
                ),
            },
        });
    };
    const addState = async (x: number, y: number) => {
        if (!currentTab || currentTab.type === "SETTINGS") return;

        const nextIndex = currentTab.automaton.states.length;
        const payload: StatePayload = {
            type: currentTab.type,
            label: `q${nextIndex}`,
            x,
            y,
            isInitial: false,
            isFinal: false,
        } as StatePayload;

        try {
            const response = await service.addState(currentTab.id, payload);

            const newTabData: AppTab = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    states: [ ...currentTab.automaton.states, response.state ],
                },
            };
            updateTab(newTabData);
        } catch (error) {
            console.error("Ошибка при добавлении вершины:", error);
        }
    };
    const removeState = async (id: number) => {
        const connectedTransitions = currentTab.automaton.transitions.filter(
            (t) => t.from === id || t.to === id,
        );

        try {
            await Promise.all(
                connectedTransitions.map((t) =>
                    service.removeTransition(currentTab.id, t.id),
                ),
            );

            await service.removeState(currentTab.id, id);
            const connectedIds: number[] = connectedTransitions.map((t) => t.id);
            const newTabData = {
                ...currentTab,
                selectedNodeId: currentTab.selectedNodeId === id ? null : currentTab.selectedNodeId,
                automaton: {
                    ...currentTab.automaton,
                    states: currentTab.automaton.states.filter((state) => state.id !== id),
                    transitions: currentTab.automaton.transitions.filter(
                        (t) => !connectedIds.includes(t.id),
                    ),
                },
            };
            updateTab(newTabData);
        } catch (error) {
            console.error("Ошибка при удалении состояния:", error);
        }
    };

    const fetchUpdateState = async (automatonId: number, stateId: number, payload: StatePayload) => {
        if (!currentTab) return;
        try {
            const response = await service.updateState(automatonId, stateId, payload);
            const newTabData: AppTab = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    states: currentTab.automaton.states.map((s) =>
                        s.id === stateId ? response.state : s,
                    ),
                },
            };
            updateTab(newTabData);
        } catch (error) {
            console.error("Ошибка при обновлении состояния:", error);
        }
    };

    const saveStatePosition = async (id: number, x: number, y: number ) => {
        if (!currentTab || currentTab.type === "SETTINGS") return;

        const payload = {
            type: currentTab.type,
            x,
            y,
        };

        try {
            const response = await service.updateState(
                currentTab.id,
                id,
                payload,
            );
            const newTabData: AppTab = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    states: currentTab.automaton.states.map((state) =>
                        state.id === id ? response.state : state,
                    ),
                },
            };
            updateTab(newTabData);
        } catch (error) {
            console.error("Ошибка при сохранении позиции вершины:", error);
        }
    };

    const handleNameChange = async (name: string, stateId: number | undefined) => {
        if (!currentTab) return;
        if (stateId === undefined || stateId === null) return;
        if (!currentTab || currentTab.type === "SETTINGS") return;

        if (name.length === 0) {
            showSnackbar("error", "Длина должна быть больше 0");
            return;
        }
        const payload = {
            type: currentTab.type,
            label: name,
        } as StatePayload;

        await fetchUpdateState(currentTab.id, stateId, payload);
    };

    const handleInitialChange = async (isInitial: boolean, stateId: number | undefined) => {
        if (!currentTab || currentTab.type === "SETTINGS") return;
        if (stateId === undefined || stateId === null) return;


        const oldInitialStateId: number | undefined = currentTab.automaton.states.find((state) => state.isInitial)?.id;

        const payload = {
            type: currentTab.type,
            isInitial: isInitial,
        };

        try {
            const response = await service.updateState(currentTab.id, stateId, payload);
            const newTabData: AppTab = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    states: currentTab.automaton.states.map((s) => {
                        if (s.id === stateId) {
                            return response.state;
                        }

                        else if (oldInitialStateId !== undefined && s.id === oldInitialStateId) {
                            showSnackbar("warning", "Начальное состояние было переопределено");
                            return { ...s, isInitial: false };
                        }

                        else {
                            return s;
                        }
                    }),
                },
            };
            updateTab(newTabData);
        } catch (error) {
            console.error("Ошибка при обновлении состояния:", error);
        }

    };

    const handleFinalChange = async (isFinal: boolean, stateId: number | undefined) => {
        if (!currentTab) return;
        if (stateId === undefined || stateId === null) return;
        if (!currentTab || currentTab.type === "SETTINGS") return;

        const payload = {
            type: currentTab.type,
            isFinal,
        } as StatePayload;

        await fetchUpdateState(currentTab.id, stateId, payload);
    };

    const handleXChange = async (x: string) => {
        if (!currentTab || currentTab.type === "SETTINGS") return;

        const newValue = parseInt(x);

        if (selectedStateId === null) return;

        const payload = {
            type: currentTab.type,
            x: newValue,
        } as StatePayload;

        await fetchUpdateState(currentTab.id, selectedStateId, payload);
    };

    const handleYChange = async (y: string) => {

        const newValue = parseInt(y);

        if (selectedStateId === null) return;

        if (!currentTab || currentTab.type === "SETTINGS") return;

        const payload = {
            type: currentTab.type,
            y: newValue,
        } as StatePayload;

        await fetchUpdateState(currentTab.id, selectedStateId, payload);
    };

    return { handleFinalChange, handleNameChange, handleXChange, handleYChange, handleInitialChange, addState, removeState, saveStatePosition, moveStateLocal };
};