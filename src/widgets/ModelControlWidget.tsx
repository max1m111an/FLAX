import Cursor from "@/assets/svg/Cursor.svg?react";
import Circle from "@/assets/svg/Circle.svg?react";
import ArrowUpRight from "@/assets/svg/ArrowUpRight.svg?react";
import Trashcan from "@/assets/svg/Trashcan.svg?react";
import Save from "@/assets/svg/Save.svg?react";
import PenLine from "@/assets/svg/PenLine.svg?react";
import Wrench from "@/assets/svg/Wrench.svg?react";
import Image from "@/assets/svg/Image.svg?react";
import Play from "@/assets/svg/Play.svg?react";


export default function ModelControlWidget() {
    return (
        <div className="model-left-control-wrapper">
            <div className="top-group">
                <Cursor className="control-icon" />
                <Circle className="control-icon" />
                <ArrowUpRight className="control-icon" />
                <Trashcan className="control-icon trashcan" />
                <div className="divider" />
            </div>

            <div className="middle-group">
                <Save className="control-icon" />
                <PenLine className="control-icon" />
                <Wrench className="control-icon" />
                <Image className="control-icon" />
            </div>

            <div className="bottom-group">
                <Play className="control-icon play" />
            </div>
        </div>
    );
}