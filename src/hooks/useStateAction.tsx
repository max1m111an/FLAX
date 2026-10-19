import { tab, useCurrentTab, useTabs } from "@/context/TabsContext.tsx";
import { updateStateFARequest } from "@/api/faAPI.ts";
import { updateState } from "@/services/faService.ts";
import { useSnackbar } from "@/context/SnackbarContext.tsx";


export const useStateActions = () => {
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();
    const { showSnackbar } = useSnackbar();


    const selectedStateId = currentTab?.selectedNodeId ?? null;

    const fetchUpdateState = async (request: updateStateFARequest) => {
        if (!currentTab) return;
        try {
            const response = await updateState(request);
            const newTabData: tab = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    states: currentTab.automaton.states.map((s) =>
                        s.id === request.stateId ? response.state : s,
                    ),
                },
            };
            updateTab(newTabData);
        } catch (error) {
            console.error("Ошибка при обновлении состояния:", error);
        }
    };

    const handleNameChange = async (name: string, stateId: number | undefined) => {
        if (!currentTab) return;
        if (stateId === undefined || stateId === null) return;
        if (name.length === 0) {
            showSnackbar("error", "Длина должна быть больше 0");
            return;
        }
        const request: updateStateFARequest = {
            automatonId: currentTab.id,
            stateId,
            label: name,
        };
        await fetchUpdateState(request);
    };

    const handleInitialChange = async (isInitial: boolean, stateId: number | undefined) => {
        if (!currentTab) return;
        if (stateId === undefined || stateId === null) return;

        const oldInitialStateId: number | undefined = currentTab.automaton.states.find((state) => state.isInitial)?.id;

        const request: updateStateFARequest = {
            automatonId: currentTab.id,
            stateId,
            isInitial,
        };
        try {
            const response = await updateState(request);
            const newTabData: tab = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    states: currentTab.automaton.states.map((s) => {
                        if (s.id === request.stateId) {
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

        const request: updateStateFARequest = {
            automatonId: currentTab.id,
            stateId,
            isFinal,
        };

        await fetchUpdateState(request);
    };

    const handleXChange = async (x: string) => {
        if (!currentTab) return;

        const newValue = parseInt(x);

        if (selectedStateId === null) return;

        const request: updateStateFARequest = {
            automatonId: currentTab.id,
            stateId: selectedStateId,
            x: newValue,
        };

        await fetchUpdateState(request);
    };

    const handleYChange = async (y: string) => {
        if (!currentTab) return;

        const newValue = parseInt(y);

        if (selectedStateId === null) return;

        const request: updateStateFARequest = {
            automatonId: currentTab.id,
            stateId: selectedStateId,
            y: newValue,
        };

        await fetchUpdateState(request);
    };

    return { handleFinalChange, handleNameChange, handleXChange, handleYChange, handleInitialChange };
};