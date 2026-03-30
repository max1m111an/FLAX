import Play from "@/assets/svg/Play.svg?react";
import FileDown from "@/assets/svg/FileDown.svg?react";

export default function ModelMultiTestingWidget() {
    return (
        <>
            <p className="model-input-label">Входные строки</p>
            <textarea
                rows={ 5 }
                className="model-multiline-input"
            />
            <div className="model-play-export-wrapper">
                <button className="model-play-multi-test-btn">
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