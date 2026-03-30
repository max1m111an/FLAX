import { useControl } from "@/context/ControlContext.tsx";

export default function ModelPropertiesNodeWidget() {
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
            <p className="state-label">Выберите вершину...</p>
        );
    }
    return (
        <>
            <p className="model-pretitle">Имя состояния</p>
            <input
                className="input"
                value={ nodes.find((node) => node.id === selectedNode)?.name || "" }
                onChange={ (e) => handleNameChange(e.currentTarget.value) }
            />

            <div className="state-wrapper">
                <p className="state-label">Начальное состояние</p>
                <label className="switch">
                    <input
                        type="checkbox"
                        checked={ nodes.find((node) => node.id === selectedNode)?.isInitial }
                        disabled={ nodes.find((node) => node.id === selectedNode)?.isFinal }
                        onChange={ handleInitialChange }
                    />
                    <span className="slider round"></span>
                </label>
            </div>
            <div className="state-wrapper">
                <p className="state-label">Финальное состояние</p>
                <label className="switch">
                    <input
                        type="checkbox"
                        checked={ nodes.find((node) => node.id === selectedNode)?.isFinal }
                        disabled={ nodes.find((node) => node.id === selectedNode)?.isInitial }
                        onChange={ handleFinalChange }
                    />
                    <span className="slider round"></span>
                </label>
            </div>
            <div className="spacer"></div>
            <p className="model-pretitle">Позиция</p>
            <div className="coordinates-wrapper">
                <div className="coordinates-label-wrapper">
                    <p className="state-label">X</p>
                    <input
                        type="number"
                        className="input"
                        value={ nodes.find((node) => node.id === selectedNode)?.x }
                        onChange={ (e) => handleXChange(e.currentTarget.value) }
                    />
                </div>
                <div className="coordinates-label-wrapper">
                    <p className="state-label">Y</p>
                    <input
                        type="number"
                        className="input"
                        value={ nodes.find((node) => node.id === selectedNode)?.y }
                        onChange={ (e) => handleYChange(e.currentTarget.value) }
                    />
                </div>
            </div>
        </>
    );
}