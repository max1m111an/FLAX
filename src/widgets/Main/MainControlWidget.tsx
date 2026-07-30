import FolderOpen from "@/assets/svg/FolderOpen.svg?react";
import Settings from "@/assets/svg/Settings.svg?react";
import Documentation from "@/assets/svg/Documentation.svg?react";
import { model } from "@/data/models.ts";
import { useTabs } from "@/context/TabsContext.tsx";
import styles from "../../scenes/MainScene.module.scss";
import { NavLink } from "react-router-dom";
import { ROUTES } from "@/configs/RoutesConst.ts";

export default function MainControlWidget() {
    const { addTab } = useTabs();
    const isDebugEnabled = import.meta.env.VITE_ENABLE_DEBUG === "true";
    const settingsModel: model = {
        id: 4,
        type: "Настройки",
        icon: Settings,
        description: "",
    };
    return (
        <div className={ styles.controlWrapper }>
            <button className={ styles.openFileButton }>
                <FolderOpen />
                Открыть файл (.jff)
            </button>
            <button className={ styles.controlButton }>
                <Documentation />
                Документация
            </button>
            <button className={ styles.controlButton } onClick={ (_e) => addTab(settingsModel, "Настройки") }>
                <Settings />
                Настройки
            </button>
            {isDebugEnabled && (
                <NavLink className={ styles.controlButton } to={ ROUTES.DEBUG }>
                    Дебаг
                </NavLink>
            )}
        </div>
    );
}