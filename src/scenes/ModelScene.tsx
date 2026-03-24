import ModelControlWidget from "@/widgets/ModelControlWidget.tsx";
import ModelCanvasWidget from "@/widgets/ModelCanvasWidget.tsx";


export default function ModelScene() {
    return (
        <div className="model-container">
            <ModelControlWidget />
            <ModelCanvasWidget />
        </div>
    );
}