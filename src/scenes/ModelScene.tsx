import ModelControlWidget from "@/widgets/ModelControlWidget.tsx";
import ModelCanvasWidget from "@/widgets/ModelCanvasWidget.tsx";
import ModelTestWidget from "@/widgets/ModelTestWidget.tsx";
import { useControl } from "@/context/ControlContext.tsx";
import ModelPropertiesWidget from "@/widgets/ModelPropertiesWidget.tsx";


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