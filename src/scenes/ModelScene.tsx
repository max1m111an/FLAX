import ModelControlWidget from "@/widgets/Model/ModelControlWidget.tsx";
import ModelCanvasWidget from "@/widgets/Model/ModelCanvasWidget.tsx";
import ModelTestWidget from "@/widgets/Model/ModelTesting/ModelTestWidget.tsx";
import styles from "./ModelScene.module.scss";
import { useCurrentTab } from "@/context/TabsContext.tsx";

export default function ModelScene() {
    const currentTab = useCurrentTab();
    if (!currentTab || currentTab.type === "SETTINGS") return null;

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
        </div>
    );
}