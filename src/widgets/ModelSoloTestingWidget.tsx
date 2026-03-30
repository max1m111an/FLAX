import Play from "@/assets/svg/Play.svg?react";
import FastForward from "@/assets/svg/FastForward.svg?react";
import Forward from "@/assets/svg/Forward.svg?react";
import Reset from "@/assets/svg/Reset.svg?react";


export default function ModelSoloTestingWidget() {
    return (
        <>
            <p className="model-input-label">Входная строка</p>
            <input className="model-solo-input" />
            <button className="model-play-solo-test-btn">
                <Play />
                Запустить
            </button>
            <div className="model-step-fast-wrapper">
                <button className="model-control-test-btn">
                    <Forward />
                    Шаг
                </button>
                <button className="model-control-test-btn">
                    <FastForward />
                    До конца
                </button>
            </div>
            <div className="model-reset-btn-wrapper">
                <button className="model-control-test-btn w-100">
                    <Reset />
                    Сброс
                </button>
            </div>
            <div className="test-state-wrapper">
                <div className="title-step-wrapper">
                    <p className="position-lbl">Позиция</p>
                    <p className="step-lbl">5/7</p>
                </div>
                <div className="line-state-wrapper">
                    <div className="states-wrapper read">
                        <p className="state">1</p>
                    </div><div className="states-wrapper read">
                        <p className="state">0</p>
                    </div>
                    <div className="states-wrapper current">
                        <p className="state current">1</p>
                    </div>
                    <div className="states-wrapper">
                        <p className="state">0</p>
                    </div>
                    <div className="states-wrapper">
                        <p className="state">1</p>
                    </div>
                    <div className="states-wrapper">
                        <p className="state">0</p>
                    </div>
                    <div className="states-wrapper">
                        <p className="state">0</p>
                    </div>
                </div>
                <p className="state-lbl">Состояние: q0</p>
            </div>
        </>
    );
}