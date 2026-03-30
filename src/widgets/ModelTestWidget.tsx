import Cancel from "@/assets/svg/Cancel.svg?react";
import { useState } from "react";
import ModelSoloTestingWidget from "@/widgets/ModelSoloTestingWidget.tsx";
import ModelFileTestingWidget from "@/widgets/ModelFileTestingWidget.tsx";
import ModelMultiTestingWidget from "@/widgets/ModelMultiTestingWidget.tsx";


export default function ModelTestWidget() {
    const [ typeTest, setTypeTest ] = useState<string>("solo");
    return (
        <div className="model-test-wrapper">
            <div className="model-title-cancel-wrapper">
                <p className="model-testing-title">Тестирование</p>
                <Cancel className="model-cancel-icon" />
            </div>
            <div className="model-switch-wrapper">
                <div className={ `switch-indicator ${typeTest}` } />
                <button onClick={ () => setTypeTest("solo") } className="model-switch-btn">
                    Единичный
                </button>
                <button onClick={ () => setTypeTest("multi") } className="model-switch-btn">
                    Мульти
                </button>
            </div>
            <div className="test-type-wrapper">
                {typeTest === "solo" && (
                    <ModelSoloTestingWidget />
                )}
                {typeTest === "multi" && (
                    <>
                        <ModelMultiTestingWidget />
                        <ModelFileTestingWidget />
                    </>
                )}
            </div>
        </div>
    );
}