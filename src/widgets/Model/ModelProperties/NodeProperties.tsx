import { useControl } from "@/context/ControlContext.tsx";
import { Textfield } from "@/components/ui/Textfield/Textfield.tsx";
import { Switch } from "@/components/ui/Switch/Switch.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import styles from "./ModelProperties.module.scss";

export default function NodeProperties() {
    const { selectedNode, nodes, setNodes } = useControl();

    const handleNameChange = (name: string) => {
        setNodes((prev) => prev.map((node) =>
            node.id === selectedNode
                ? { ...node, name }
                : node,
        ));
    };

    const handleInitialChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = e.target.checked;
        setNodes((prev) => prev.map((node) =>
            node.id === selectedNode
                ? { ...node, isInitial: newValue }
                : node,
        ));
    };

    const handleFinalChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = e.target.checked;
        setNodes((prev) => prev.map((node) =>
            node.id === selectedNode
                ? { ...node, isFinal: newValue }
                : node,
        ));
    };

    const handleXChange = (x: string) => {
        const newValue = parseInt(x);

        setNodes((prev) => prev.map((node) =>
            node.id === selectedNode
                ? { ...node, x: newValue }
                : node,
        ));
    };
    const handleYChange = (y: string) => {
        const newValue = parseInt(y);
        setNodes((prev) => prev.map((node) =>
            node.id === selectedNode
                ? { ...node, y: newValue }
                : node,
        ));
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
                value={ nodes.find((node) => node.id === selectedNode)?.name || "" }
                onChange={ (e) => handleNameChange(e.currentTarget.value) }
            />

            <div className={ styles.stateWrapper }>
                <Typography variant="label">Начальное состояние</Typography>
                <Switch
                    checked={ nodes.find((node) => node.id === selectedNode)?.isInitial }
                    disabled={ nodes.find((node) => node.id === selectedNode)?.isFinal }
                    onChange={ handleInitialChange }
                />
            </div>
            <div className={ styles.stateWrapper }>
                <Typography variant="label">Финальное состояние</Typography>
                <Switch
                    checked={ nodes.find((node) => node.id === selectedNode)?.isFinal }
                    disabled={ nodes.find((node) => node.id === selectedNode)?.isInitial }
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
                        value={ nodes.find((node) => node.id === selectedNode)?.x }
                        onChange={ (e) => handleXChange(e.currentTarget.value) }
                    />
                </div>
                <div className={ styles.coordinatesLabelWrapper }>
                    <Typography variant="label">Y</Typography>
                    <Textfield
                        type="number"
                        value={ nodes.find((node) => node.id === selectedNode)?.y }
                        onChange={ (e) => handleYChange(e.currentTarget.value) }
                    />
                </div>
            </div>
        </>
    );
}