import Play from "@/assets/svg/Play.svg?react";
import FileDown from "@/assets/svg/FileDown.svg?react";

export default function ModelMultiTestingWidget() {
    return (
        <>
            <p className="model-pretitle">Входные строки</p>
            <textarea
                rows={ 5 }
                className="multiline-input"
            />
            <div className="model-play-export-wrapper">
                <button className="main-btn">
                    <Play />
                    Запустить все
                </button>
                <button className="model-export-multi-test-btn">
                    <FileDown />
                    Экспорт
                </button>
            </div>
        </>
    );
}