import ModelControlWidget from "@/widgets/ModelControlWidget.tsx";
import ModelCanvasWidget from "@/widgets/ModelCanvasWidget.tsx";
import ModelTestWidget from "@/widgets/ModelTestWidget.tsx";
import { useControl } from "@/context/RightControlContext.tsx";
import ModelPropertiesWidget from "@/widgets/ModelPropertiesWidget.tsx";


export default function ModelScene() {
    const { activeControl } = useControl();
    return (
        <div className="model-container">
            <ModelControlWidget />
            <ModelCanvasWidget />
            {activeControl == "play" && (
                <ModelTestWidget />
            )
            }
            {activeControl == "settings" && (
                <ModelPropertiesWidget />
            )
            }
        </div>
    );
}