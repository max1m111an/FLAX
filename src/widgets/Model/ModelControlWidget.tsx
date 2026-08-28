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


export default function ModelControlWidget() {
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();

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
                <Save className={ styles.modelControlIcon } />
                <PenLine className={ styles.modelControlIcon } />
                <Wrench className={ styles.modelControlIcon } />
                <Image className={ styles.modelControlIcon } />
            </div>

            <div className={ styles.modelBottomGroup }>
                <Settings
                    className={ clsx(styles.modelControlIcon, currentTab?.activePane == "settings" && styles.active) }
                    onClick={ () => {
                        if (currentTab) {
                            if (currentTab.activePane !== "settings") {
                                updateTab({
                                    ...currentTab,
                                    activePane: "settings",
                                });
                            } else {
                                updateTab({
                                    ...currentTab,
                                    activePane: null,
                                });
                            }
                        }
                    } } />
                <Play
                    className={ clsx(styles.modelControlIcon, currentTab?.activePane == "play" && styles.active) }
                    onClick={ () => {
                        if (currentTab) {
                            if (currentTab.activePane !== "play") {
                                updateTab({
                                    ...currentTab,
                                    activePane: "play",
                                });
                            } else {
                                updateTab({
                                    ...currentTab,
                                    activePane: null,
                                });
                            }
                        }
                    } } />
            </div>
        </div>
    );
}