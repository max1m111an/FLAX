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
import { useControl } from "@/context/ControlContext.tsx";
import clsx from "clsx";
import styles from "../../scenes/ModelScene.module.scss";


export default function ModelControlWidget() {
    const { activePane, changePane } = useControl();
    const { activeControl, changeControl } = useControl();
    return (
        <div className={ styles.modelLeftControlWrapper }>
            <div className={ styles.modelTopGroup }>
                <Cursor className={ clsx(styles.modelControlIcon, activeControl == "cursor" && styles.active) }
                    onClick={ () => changeControl("cursor") } />
                <Move className={ clsx(styles.modelControlIcon,
                    activeControl == "Move" && styles.active) } onClick={ () => changeControl("Move") } />
                <Circle className={ clsx(styles.modelControlIcon,
                    activeControl == "node" && styles.active) } onClick={ () => changeControl("node") } />
                <ArrowUpRight className={ clsx(styles.modelControlIcon,
                    activeControl == "edge" && styles.active) } onClick={ () => changeControl("edge") } />
                <Trashcan className={ clsx(styles.modelControlIcon, styles.trashcan, activeControl == "trashcan" && styles.active) }
                    onClick={ () => changeControl("trashcan") } />
                <div className={ styles.modelDivider } />
            </div>

            <div className={ styles.modelMiddleGroup }>
                <Save className={ styles.modelControlIcon } />
                <PenLine className={ styles.modelControlIcon } />
                <Wrench className={ styles.modelControlIcon } />
                <Image className={ styles.modelControlIcon } />
            </div>

            <div className={ styles.modelBottomGroup }>
                <Settings className={ clsx(styles.modelControlIcon, activePane == "settings" && styles.active) } onClick={ () => changePane("settings") } />
                <Play className={ clsx(styles.modelControlIcon, activePane == "play" && styles.active) } onClick={ () => changePane("play") } />
            </div>
        </div>
    );
}