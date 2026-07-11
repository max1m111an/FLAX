import FolderOpen from "@/assets/svg/FolderOpen.svg?react";
import Settings from "@/assets/svg/Settings.svg?react";
import Documentation from "@/assets/svg/Documentation.svg?react";
import { model } from "@/data/models.ts";
import { useTabs } from "@/context/TabsContext.tsx";

export default function MainControlWidget() {
    const { addTab } = useTabs();

    const settingsModel: model = {
        id: 4,
        type: "Настройки",
        icon: Settings,
        description: "",
    };
    return (
        <div className="control-wrapper">
            <button className="open-file-button">
                <FolderOpen />
                Открыть файл (.jff)
            </button>
            <button className="control-button">
                <Documentation />
                Документация
            </button>
            <button className="control-button" onClick={ (_e) => addTab(settingsModel, "Настройки") }>
                <Settings />
                Настройки
            </button>
        </div>
    );
}