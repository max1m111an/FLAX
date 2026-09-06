import Cursor from "@/assets/svg/Cursor.svg?react";
import Circle from "@/assets/svg/Circle.svg?react";
import ArrowUpRight from "@/assets/svg/ArrowUpRight.svg?react";
import Trashcan from "@/assets/svg/Trashcan.svg?react";
import Save from "@/assets/svg/Save.svg?react";
import PenLine from "@/assets/svg/PenLine.svg?react";
import Wrench from "@/assets/svg/Wrench.svg?react";
import Image from "@/assets/svg/Image.svg?react";
import Play from "@/assets/svg/Play.svg?react";
import Settings from "@/assets/svg/Settings.svg?react";
import Move from "@/assets/svg/Move.svg?react";
import clsx from "clsx";
import styles from "../../scenes/ModelScene.module.scss";
import { useCurrentTab, useTabs } from "@/context/TabsContext.tsx";
import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { save } from "@tauri-apps/plugin-dialog";
import { saveJff } from "@/services/jffService.ts";
import { isDirty } from "@/services/dirtyState.ts";
import { basename } from "@tauri-apps/api/path";


export default function ModelControlWidget() {
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();

    const fetchSave = async (): Promise<boolean> => {
        try {
            const filePath = await save({
                defaultPath: `${currentTab?.title}.jff`,
                filters: [ { name: "Единый формат .jff", extensions: [ "jff" ] } ],
            });

            if (!filePath || currentTab === undefined) return false;

            const fileName = await basename(filePath);
            const nameWithoutExt = fileName.replace(/\.jff$/i, "");

            await saveJff({
                automatonId: currentTab.id,
                path: filePath,
            });

            updateTab({
                ...currentTab,
                title: nameWithoutExt,
                isSaved: true,
                savedPath: filePath,
            });
            return true;
        } catch (error) {
            console.error("Ошибка при сохранении файла:", error);
            return false;
        }
    };
    const handleSaveJFF = async () => {
        if (currentTab?.isSaved && !isDirty(currentTab.id)) {
            return;
        }
        const filePath = currentTab?.savedPath;
        if (filePath) {
            await saveJff({ automatonId: currentTab.id, path: filePath });
            return;
        }
        fetchSave();
    };
    const handleSaveAsJFF = async () => {
        fetchSave();
    };
    return (
        <div className={ styles.modelLeftControlWrapper }>
            <div className={ styles.modelTopGroup }>
                <Cursor
                    className={ clsx(styles.modelControlIcon, currentTab?.activeControl == "cursor" && styles.active) }
                    onClick={ () => {
                        if (currentTab) {
                            updateTab({
                                ...currentTab,
                                activeControl: "cursor",
                            });
                        }
                    } } />
                <Move
                    className={ clsx(styles.modelControlIcon, currentTab?.activeControl == "move" && styles.active) }
                    onClick={ () => {
                        if (currentTab) {
                            updateTab({
                                ...currentTab,
                                activeControl: "move",
                            });
                        }
                    } } />
                <Circle
                    className={ clsx(styles.modelControlIcon, currentTab?.activeControl == "node" && styles.active) }
                    onClick={ () => {
                        if (currentTab) {
                            updateTab({
                                ...currentTab,
                                activeControl: "node",
                            });
                        }
                    } } />
                <ArrowUpRight
                    className={ clsx(styles.modelControlIcon, currentTab?.activeControl == "edge" && styles.active) }
                    onClick={ () => {
                        if (currentTab) {
                            updateTab({
                                ...currentTab,
                                activeControl: "edge",
                            });
                        }
                    } } />
                <Trashcan
                    className={ clsx(styles.modelControlIcon, styles.trashcan, currentTab?.activeControl == "trashcan" && styles.active) }
                    onClick={ () => {
                        if (currentTab) {
                            updateTab({
                                ...currentTab,
                                activeControl: "trashcan",
                            });
                        }
                    } } />
                <div className={ styles.modelDivider } />
            </div>

            <div className={ styles.modelMiddleGroup }>
                <DropdownMenu.Root>
                    <DropdownMenu.Trigger className={ styles.modelControlIcon }
                        asChild>
                        <Save />
                    </DropdownMenu.Trigger>

                    <DropdownMenu.Content className={ styles.dropdown }>
                        <DropdownMenu.Group>
                            <DropdownMenu.Item className={ styles.item } onClick={ handleSaveJFF }>
                                Сохранить
                            </DropdownMenu.Item>
                            <DropdownMenu.Item className={ styles.item } onClick={ handleSaveAsJFF }>
                                Сохранить как
                            </DropdownMenu.Item>
                        </DropdownMenu.Group>
                    </DropdownMenu.Content>
                </DropdownMenu.Root>
                <PenLine className={ styles.modelControlIcon } />
                <Wrench className={ styles.modelControlIcon } />
                <Image className={ styles.modelControlIcon } />
            </div>

            <div className={ styles.modelBottomGroup }>
                <Settings
                    className={ clsx(styles.modelControlIcon, currentTab?.activePanel == "settings" && styles.active) }
                    onClick={ () => {
                        if (currentTab) {
                            if (currentTab.activePanel !== "settings") {
                                updateTab({
                                    ...currentTab,
                                    activePanel: "settings",
                                });
                            } else {
                                updateTab({
                                    ...currentTab,
                                    activePanel: null,
                                });
                            }
                        }
                    } } />
                <Play
                    className={ clsx(styles.modelControlIcon, currentTab?.activePanel == "play" && styles.active) }
                    onClick={ () => {
                        if (currentTab) {
                            if (currentTab.activePanel !== "play") {
                                updateTab({
                                    ...currentTab,
                                    activePanel: "play",
                                });
                            } else {
                                updateTab({
                                    ...currentTab,
                                    activePanel: null,
                                });
                            }
                        }
                    } } />
            </div>
        </div>
    );
}