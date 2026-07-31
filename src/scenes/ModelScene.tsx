import ModelControlWidget from "@/widgets/Model/ModelControlWidget.tsx";
import ModelCanvasWidget from "@/widgets/Model/ModelCanvasWidget.tsx";
import ModelTestWidget from "@/widgets/Model/ModelTesting/ModelTestWidget.tsx";
import { useControl } from "@/context/ControlContext.tsx";
import ModelPropertiesWidget from "@/widgets/Model/ModelProperties/ModelPropertiesWidget.tsx";
import styles from "./ModelScene.module.scss";
import { useTabs } from "@/context/TabsContext.tsx";
import { useParams } from "react-router-dom";

export default function ModelScene() {
    const { activePane } = useControl();
    const { tabs } = useTabs();
    const { id } = useParams();

    const currentTab = tabs.find((tab) => String(tab.id) === id);

    if (!currentTab) {
        return <div className={ styles.modelContainer }>Вкладка не найдена</div>;
    }

    return (
        <div className={ styles.modelContainer }>
            <ModelControlWidget />
            <ModelCanvasWidget key={ currentTab.id } tab={ currentTab } />
            {activePane == "play" && (
                <ModelTestWidget />
            )}
            {activePane == "settings" && (
                <ModelPropertiesWidget />
            )}
        </div>
    );
}