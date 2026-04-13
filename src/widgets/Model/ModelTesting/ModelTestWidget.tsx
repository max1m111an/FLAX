import Cancel from "@/assets/svg/Cancel.svg?react";
import { useState } from "react";
import SoloTesting from "./SoloTesting.tsx";
import FileTesting from "./FileTesting.tsx";
import MultiTesting from "./MultiTesting.tsx";


export default function ModelTestWidget() {
    const [ typeTest, setTypeTest ] = useState<string>("solo");
    return (
        <div className="model-test-wrapper">
            <div className="model-title-cancel-wrapper">
                <p className="model-title">Тестирование</p>
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
                    <SoloTesting />
                )}
                {typeTest === "multi" && (
                    <>
                        <MultiTesting />
                        <FileTesting />
                    </>
                )}
            </div>
        </div>
    );
}