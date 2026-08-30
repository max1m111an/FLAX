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
            <div className={ currentTab.activePanel != "play" ? styles.hidden : undefined }>
                <ModelTestWidget key={ currentTab.id } />
            </div>
            <div className={ currentTab.activePanel != "settings" ? styles.hidden : undefined }>
                <ModelPropertiesWidget key={ currentTab.id } />
            </div>
        </div>
    );
}