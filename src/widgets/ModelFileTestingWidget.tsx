import Uploud from "@/assets/svg/Uploud.svg?react";


export default function ModelFileTestingWidget() {
    return (
        <div className="model-import-wrapper">
            <Uploud className="model-import-icon" />
            <p className="model-import-title">Выберите файл</p>
            <p className="model-import-pretitle">.txt, .cvs</p>
            <button className="model-import-btn">Обзор</button>
        </div>
    );
}