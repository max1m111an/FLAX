import { Textfield } from "@/components/ui/Textfield/Textfield.tsx";
import { Switch } from "@/components/ui/Switch/Switch.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import styles from "./ModelProperties.module.scss";
import { tab, useCurrentTab, useTabs } from "@/context/TabsContext";
import { updateState, updateStateNFARequest } from "@/services/nfaService.ts";

export default function NodeProperties() {
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();

    if (!currentTab) return null;

    const selectedStateId = currentTab.selectedNodeId;

    const fetchUpdateState = async (request: updateStateNFARequest) => {
        try {
            const response = await updateState(request);
            const newTabData: tab = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    states: currentTab.automaton.states.map((s) =>
                        s.id === selectedStateId ? response.state : s,
                    ),
                },
            };
            updateTab(newTabData);
        } catch (error) {
            console.error("Ошибка при обновлении состояния:", error);
        }
    };

    const handleNameChange = async (name: string) => {
        if (selectedStateId === null) return;
        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: selectedStateId,
            label: name,
        };
        await fetchUpdateState(request);
    };

    const handleInitialChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = e.target.checked;

        if (selectedStateId === null) return;

        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: selectedStateId,
            isInitial: newValue,
        };

        await fetchUpdateState(request);
    };

    const handleFinalChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = e.target.checked;

        if (selectedStateId === null) return;

        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: selectedStateId,
            isFinal: newValue,
        };

        await fetchUpdateState(request);
    };

    const handleXChange = async (x: string) => {
        const newValue = parseInt(x);

        if (selectedStateId === null) return;

        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: selectedStateId,
            x: newValue,
        };

        await fetchUpdateState(request);
    };
    const handleYChange = async (y: string) => {
        const newValue = parseInt(y);

        if (selectedStateId === null) return;

        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: selectedStateId,
            y: newValue,
        };

        await fetchUpdateState(request);
    };
    if (selectedStateId === null) {
        return (
            <Typography variant="label">Выберите вершину...</Typography>
        );
    }
    return (
        <>
            <Typography variant="pretitle">Имя состояния</Typography>
            <Textfield
                value={ currentTab.automaton.states.find((node) => node.id === selectedStateId)?.label || "" }
                onChange={ (e) => handleNameChange(e.currentTarget.value) }
            />

            <div className={ styles.stateWrapper }>
                <Typography variant="label">Начальное состояние</Typography>
                <Switch
                    checked={ currentTab.automaton.states.find((node) => node.id === selectedStateId)?.isInitial }
                    disabled={ currentTab.automaton.states.find((node) => node.id === selectedStateId)?.isFinal }
                    onChange={ handleInitialChange }
                />
            </div>
            <div className={ styles.stateWrapper }>
                <Typography variant="label">Финальное состояние</Typography>
                <Switch
                    checked={ currentTab.automaton.states.find((node) => node.id === selectedStateId)?.isFinal }
                    disabled={ currentTab.automaton.states.find((node) => node.id === selectedStateId)?.isInitial }
                    onChange={ handleFinalChange }
                />
            </div>
            <div className={ styles.spacer }></div>
            <Typography variant="pretitle">Позиция</Typography>
            <div className={ styles.coordinatesWrapper }>
                <div className={ styles.coordinatesLabelWrapper }>
                    <Typography variant="label">X</Typography>
                    <Textfield
                        type="number"
                        value={ currentTab.automaton.states.find((node) => node.id === selectedStateId)?.x }
                        onChange={ (e) => handleXChange(e.currentTarget.value) }
                    />
                </div>
                <div className={ styles.coordinatesLabelWrapper }>
                    <Typography variant="label">Y</Typography>
                    <Textfield
                        type="number"
                        value={ currentTab.automaton.states.find((node) => node.id === selectedStateId)?.y }
                        onChange={ (e) => handleYChange(e.currentTarget.value) }
                    />
                </div>
            </div>
        </>
    );
}