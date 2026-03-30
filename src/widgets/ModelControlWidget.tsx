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
import { useControl } from "@/context/RightControlContext.tsx";


export default function ModelControlWidget() {
    const { activeControl, changeActiveControl } = useControl();
    return (
        <div className="model-left-control-wrapper">
            <div className="model-top-group">
                <Cursor className="model-control-icon" />
                <Circle className="model-control-icon" />
                <ArrowUpRight className="model-control-icon" />
                <Trashcan className="model-control-icon trashcan" />
                <div className="model-divider" />
            </div>

            <div className="model-middle-group">
                <Save className="model-control-icon" />
                <PenLine className="model-control-icon" />
                <Wrench className="model-control-icon" />
                <Image className="model-control-icon" />
            </div>

            <div className="model-bottom-group">
                <Settings className={ `model-control-icon ${activeControl == "settings" && "active"}` } onClick={ () => changeActiveControl("settings") } />
                <Play className={ `model-control-icon ${activeControl == "play" && "active"}` } onClick={ () => changeActiveControl("play") } />
            </div>
        </div>
    );
}