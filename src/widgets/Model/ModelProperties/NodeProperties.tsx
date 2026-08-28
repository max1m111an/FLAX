import { Textfield } from "@/components/ui/Textfield/Textfield.tsx";
import { Switch } from "@/components/ui/Switch/Switch.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import styles from "./ModelProperties.module.scss";
import { tab, useCurrentTab, useTabs } from "@/context/TabsContext";
import { updateStateNFA, updateStateNFARequest } from "@/api/nfaAPI.ts";

export default function NodeProperties() {
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();

    if (!currentTab) return null;

    const fetchUpdateState = async (request: updateStateNFARequest) => {
        const response = await updateStateNFA(request);
        if (response.status == 200) {
            const newTabData: tab = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    states: currentTab.automaton.states.map((state) =>
                        state.id === currentTab.selectedState ? response.state : state,
                    ),
                },
            };
            updateTab(newTabData);
        }
    };

    const handleNameChange = async (name: string) => {
        if (currentTab.selectedState === null) return;
        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: currentTab.selectedState,
            label: name,
        };
        await fetchUpdateState(request);
    };

    const handleInitialChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = e.target.checked;

        if (currentTab.selectedState === null) return;

        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: currentTab.selectedState,
            isInitial: newValue,
        };

        await fetchUpdateState(request);
    };

    const handleFinalChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = e.target.checked;

        if (currentTab.selectedState === null) return;

        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: currentTab.selectedState,
            isFinal: newValue,
        };

        await fetchUpdateState(request);
    };

    const handleXChange = async (x: string) => {
        const newValue = parseInt(x);

        if (currentTab.selectedState === null) return;

        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: currentTab.selectedState,
            x: newValue,
        };

        await fetchUpdateState(request);
    };
    const handleYChange = async (y: string) => {
        const newValue = parseInt(y);

        if (currentTab.selectedState === null) return;

        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: currentTab.selectedState,
            y: newValue,
        };

        await fetchUpdateState(request);
    };
    if (currentTab.selectedState === null) {
        return (
            <Typography variant="label">Выберите вершину...</Typography>
        );
    }
    return (
        <>
            <Typography variant="pretitle">Имя состояния</Typography>
            <Textfield
                value={ currentTab.automaton.states.find((node) => node.id === currentTab.selectedState)?.label || "" }
                onChange={ (e) => handleNameChange(e.currentTarget.value) }
            />

            <div className={ styles.stateWrapper }>
                <Typography variant="label">Начальное состояние</Typography>
                <Switch
                    checked={ currentTab.automaton.states.find((node) => node.id === currentTab.selectedState)?.isInitial }
                    disabled={ currentTab.automaton.states.find((node) => node.id === currentTab.selectedState)?.isFinal }
                    onChange={ handleInitialChange }
                />
            </div>
            <div className={ styles.stateWrapper }>
                <Typography variant="label">Финальное состояние</Typography>
                <Switch
                    checked={ currentTab.automaton.states.find((node) => node.id === currentTab.selectedState)?.isFinal }
                    disabled={ currentTab.automaton.states.find((node) => node.id === currentTab.selectedState)?.isInitial }
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
                        value={ currentTab.automaton.states.find((node) => node.id === currentTab.selectedState)?.x }
                        onChange={ (e) => handleXChange(e.currentTarget.value) }
                    />
                </div>
                <div className={ styles.coordinatesLabelWrapper }>
                    <Typography variant="label">Y</Typography>
                    <Textfield
                        type="number"
                        value={ currentTab.automaton.states.find((node) => node.id === currentTab.selectedState)?.y }
                        onChange={ (e) => handleYChange(e.currentTarget.value) }
                    />
                </div>
            </div>
        </>
    );
}