import ModelControlWidget from "@/widgets/Model/ModelControlWidget.tsx";
import ModelCanvasWidget from "@/widgets/Model/ModelCanvasWidget.tsx";
import ModelTestWidget from "@/widgets/Model/ModelTesting/ModelTestWidget.tsx";
import { useControl } from "@/context/ControlContext.tsx";
import ModelPropertiesWidget from "@/widgets/Model/ModelProperties/ModelPropertiesWidget.tsx";


export default function ModelScene() {
    const { activePane } = useControl();
    return (
        <div className="model-container">
            <ModelControlWidget />
            <ModelCanvasWidget />
            {activePane == "play" && (
                <ModelTestWidget />
            )
            }
            {activePane == "settings" && (
                <ModelPropertiesWidget />
            )
            }
        </div>
    );
}