import FolderOpen from "@/assets/svg/FolderOpen.svg?react";
import Settings from "@/assets/svg/Settings.svg?react";
import Documentation from "@/assets/svg/Documentation.svg?react";
import { model, models } from "@/data/models.ts";
import { useTabs } from "@/context/TabsContext.tsx";
import styles from "../../scenes/MainScene.module.scss";
import { NavLink } from "react-router-dom";
import { ROUTES } from "@/configs/RoutesConst.ts";
import { open } from "@tauri-apps/plugin-dialog";
import { loadJff } from "@/services/jffService.ts";


export default function MainControlWidget() {
    const { addTab, loadTab } = useTabs();
    const isDebugEnabled = import.meta.env.VITE_ENABLE_DEBUG === "true";
    const settingsModel: model = {
        id: 4,
        type: "Настройки",
        icon: Settings,
        description: "",
    };
    const selectFile = async () => {
        try {
            const filePath = await open({
                multiple: false,
                directory: false,
                title: "Выберите файл",
                filters: [
                    {
                        name: "Файлы .jff",
                        extensions: [ "jff" ],
                    },
                ],
            });
            if (!filePath) {
                return;
            }

            const response = await loadJff({ path: filePath });
            if (response.automaton) {
                loadTab(response.automaton, models[0], filePath);
            }
        } catch (error) {
            console.error("Ошибка при выборе файла:", error);
        }
    };
    return (
        <div className={ styles.controlWrapper }>
            <button className={ styles.openFileButton } onClick={ selectFile }>
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