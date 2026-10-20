import FolderOpen from "@/assets/svg/FolderOpen.svg?react";
import Settings from "@/assets/svg/Settings.svg?react";
import Documentation from "@/assets/svg/Documentation.svg?react";
import { useTabs } from "@/context/TabsContext.tsx";
import { useRecentFiles } from "@/context/RecentFilesContext.tsx";
import styles from "../../scenes/MainScene.module.scss";
import { NavLink } from "react-router-dom";
import { ROUTES } from "@/configs/RoutesConst.ts";
import { open } from "@tauri-apps/plugin-dialog";


export default function MainControlWidget() {
    const { loadTab, addUtilTab } = useTabs();
    const { addFile } = useRecentFiles();
    const isDebugEnabled = import.meta.env.VITE_ENABLE_DEBUG === "true";

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

            loadTab(filePath);
            addFile(filePath);

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
            <button className={ styles.controlButton } onClick={ (_e) => addUtilTab("SETTINGS", Settings) }>
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