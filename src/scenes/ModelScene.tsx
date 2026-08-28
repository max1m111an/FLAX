import ModelControlWidget from "@/widgets/Model/ModelControlWidget.tsx";
import ModelCanvasWidget from "@/widgets/Model/ModelCanvasWidget.tsx";
import ModelTestWidget from "@/widgets/Model/ModelTesting/ModelTestWidget.tsx";
import ModelPropertiesWidget from "@/widgets/Model/ModelProperties/ModelPropertiesWidget.tsx";
import styles from "./ModelScene.module.scss";
import { useCurrentTab } from "@/context/TabsContext.tsx";

export default function ModelScene() {
    const currentTab = useCurrentTab();

    if (!currentTab) {
        return <div className={ styles.modelContainer }>Вкладка не найдена</div>;
    }

    return (
        <div className={ styles.modelContainer }>
            <ModelControlWidget />
            <ModelCanvasWidget key={ currentTab.id } />
            {currentTab.activePane == "play" && (
                <ModelTestWidget />
            )}
            {currentTab.activePane == "settings" && (
                <ModelPropertiesWidget key={ currentTab.id } />
            )}
        </div>
    );
}