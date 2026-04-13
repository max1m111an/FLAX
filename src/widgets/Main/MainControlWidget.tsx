import FolderOpen from "@/assets/svg/FolderOpen.svg?react";
import Settings from "@/assets/svg/Settings.svg?react";
import Documentation from "@/assets/svg/Documentation.svg?react";

export default function MainControlWidget() {
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
            <button className="control-button">
                <Settings />
                Настройки
            </button>
        </div>
    );
}