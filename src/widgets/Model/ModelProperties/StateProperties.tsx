import { Textfield } from "@/components/ui/Textfield/Textfield.tsx";
import { Switch } from "@/components/ui/Switch/Switch.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import styles from "./ModelProperties.module.scss";
import { useCurrentTab } from "@/context/TabsContext";
import { useStateActions } from "@/hooks/useStateAction.tsx";

export default function StateProperties() {
    const currentTab = useCurrentTab();
    const { handleFinalChange, handleNameChange, handleXChange, handleYChange, handleInitialChange } = useStateActions();

    if (!currentTab) return null;

    const selectedStateId = currentTab.selectedNodeId;

    const selectedState = currentTab.automaton.states.find(
        (node) => node.id === selectedStateId,
    );
    if (selectedStateId === null) {
        return (
            <Typography variant="label">Выберите вершину...</Typography>
        );
    }
    return (
        <>
            <Typography variant="pretitle">Имя состояния</Typography>
            <Textfield
                value={ selectedState?.label || "" }
                onChange={ (e) => handleNameChange(e.currentTarget.value) }
            />

            <div className={ styles.stateWrapper }>
                <Typography variant="label">Начальное состояние</Typography>
                <Switch
                    checked={ selectedState?.isInitial }
                    disabled={ selectedState?.isFinal }
                    onChange={ (e) => handleInitialChange(e.target.checked, selectedState?.id) }
                />
            </div>
            <div className={ styles.stateWrapper }>
                <Typography variant="label">Финальное состояние</Typography>
                <Switch
                    checked={ selectedState?.isFinal }
                    disabled={ selectedState?.isInitial }
                    onChange={ (e) => handleFinalChange(e.target.checked, selectedState?.id) }
                />
            </div>
            <div className={ styles.spacer }></div>
            <Typography variant="pretitle">Позиция</Typography>
            <div className={ styles.coordinatesWrapper }>
                <div className={ styles.coordinatesLabelWrapper }>
                    <Typography variant="label">X</Typography>
                    <Textfield
                        type="number"
                        value={ selectedState?.x }
                        onChange={ (e) => handleXChange(e.currentTarget.value) }
                    />
                </div>
                <div className={ styles.coordinatesLabelWrapper }>
                    <Typography variant="label">Y</Typography>
                    <Textfield
                        type="number"
                        value={ selectedState?.y }
                        onChange={ (e) => handleYChange(e.currentTarget.value) }
                    />
                </div>
            </div>
        </>
    );
}