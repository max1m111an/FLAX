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
    const { activePane, changePane } = useControl();
    const { activeControl, changeControl } = useControl();
    return (
        <div className="model-left-control-wrapper">
            <div className="model-top-group">
                <Cursor className={`model-control-icon ${activeControl == "cursor" && "active"}`} onClick={ () => changeControl("cursor") }/>
                <Circle className={`model-control-icon ${activeControl == "node" && "active"}`} onClick={ () => changeControl("node") }/>
                <ArrowUpRight className={`model-control-icon ${activeControl == "edge" && "active"}`} onClick={ () => changeControl("edge") }/>
                <Trashcan className={`model-control-icon trashcan ${activeControl == "trashcan" && "active"}`} onClick={ () => changeControl("trashcan") }/>
                <div className="model-divider" />
            </div>

            <div className="model-middle-group">
                <Save className="model-control-icon" />
                <PenLine className="model-control-icon" />
                <Wrench className="model-control-icon" />
                <Image className="model-control-icon" />
            </div>

            <div className="model-bottom-group">
                <Settings className={ `model-control-icon ${activePane == "settings" && "active"}` } onClick={ () => changePane("settings") } />
                <Play className={ `model-control-icon ${activePane == "play" && "active"}` } onClick={ () => changePane("play") } />
            </div>
        </div>
    );
}