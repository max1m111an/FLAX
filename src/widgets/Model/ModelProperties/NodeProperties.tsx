import { useControl } from "@/context/ControlContext.tsx";
import { Textfield } from "@/components/ui/Textfield/Textfield.tsx";
import { Switch } from "@/components/ui/Switch/Switch.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import styles from "./ModelProperties.module.scss";
import { useEffect, useState } from "react";
import { tab, useTabs } from "@/context/TabsContext";
import { updateStateNFA, updateStateNFARequest } from "@/api/nfaAPI.ts";

interface NodePropertiesProps {
    tab: tab;
}

export default function NodeProperties({ tab }: NodePropertiesProps) {
    const { selectedNode } = useControl();
    const [ currentTab, setCurrentTab ] = useState<tab>(tab);
    const { updateTab } = useTabs();

    useEffect(() => {
        setCurrentTab(tab);
    }, [ tab ]);

    const fetchUpdateState = async (request: updateStateNFARequest) => {
        const response = await updateStateNFA(request);
        if (response.status == 200) {
            const newTabData: tab = {
                ...currentTab,
                automaton: {
                    ...currentTab.automaton,
                    states: currentTab.automaton.states.map((state) =>
                        state.id === selectedNode ? response.state : state,
                    ),
                },
            };
            setCurrentTab(newTabData);
            updateTab(newTabData);
        }
    };

    const handleNameChange = async (name: string) => {
        if (selectedNode === null) return;
        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: selectedNode,
            label: name,
        };
        await fetchUpdateState(request);
    };

    const handleInitialChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = e.target.checked;

        if (selectedNode === null) return;

        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: selectedNode,
            isInitial: newValue,
        };

        await fetchUpdateState(request);
    };

    const handleFinalChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = e.target.checked;

        if (selectedNode === null) return;

        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: selectedNode,
            isFinal: newValue,
        };

        await fetchUpdateState(request);
    };

    const handleXChange = async (x: string) => {
        const newValue = parseInt(x);

        if (selectedNode === null) return;

        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: selectedNode,
            x: newValue,
        };

        await fetchUpdateState(request);
    };
    const handleYChange = async (y: string) => {
        const newValue = parseInt(y);

        if (selectedNode === null) return;

        const request: updateStateNFARequest = {
            automatonId: currentTab.id,
            stateId: selectedNode,
            y: newValue,
        };

        await fetchUpdateState(request);
    };
    if (selectedNode === null) {
        return (
            <Typography variant="label">Выберите вершину...</Typography>
        );
    }
    return (
        <>
            <Typography variant="pretitle">Имя состояния</Typography>
            <Textfield
                value={ currentTab.automaton.states.find((node) => node.id === selectedNode)?.label || "" }
                onChange={ (e) => handleNameChange(e.currentTarget.value) }
            />

            <div className={ styles.stateWrapper }>
                <Typography variant="label">Начальное состояние</Typography>
                <Switch
                    checked={ currentTab.automaton.states.find((node) => node.id === selectedNode)?.isInitial }
                    disabled={ currentTab.automaton.states.find((node) => node.id === selectedNode)?.isFinal }
                    onChange={ handleInitialChange }
                />
            </div>
            <div className={ styles.stateWrapper }>
                <Typography variant="label">Финальное состояние</Typography>
                <Switch
                    checked={ currentTab.automaton.states.find((node) => node.id === selectedNode)?.isFinal }
                    disabled={ currentTab.automaton.states.find((node) => node.id === selectedNode)?.isInitial }
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
                        value={ currentTab.automaton.states.find((node) => node.id === selectedNode)?.x }
                        onChange={ (e) => handleXChange(e.currentTarget.value) }
                    />
                </div>
                <div className={ styles.coordinatesLabelWrapper }>
                    <Typography variant="label">Y</Typography>
                    <Textfield
                        type="number"
                        value={ currentTab.automaton.states.find((node) => node.id === selectedNode)?.y }
                        onChange={ (e) => handleYChange(e.currentTarget.value) }
                    />
                </div>
            </div>
        </>
    );
}