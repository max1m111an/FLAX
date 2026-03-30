import Cancel from "@/assets/svg/Cancel.svg?react";
import { useState } from "react";
import ModelPropertiesNodeWidget from "@/widgets/ModelPropertiesNodeWidget.tsx";
import ModelPropertiesEdgeWidget from "@/widgets/ModelPropertiesEdgeWidget.tsx";


export default function ModelPropertiesWidget() {
    const [ typeProp, setTypeProp ] = useState<string>("nodeType");

    return (
        <div className="model-test-wrapper">
            <div className="model-title-cancel-wrapper">
                <p className="model-title">Свойства</p>
                <Cancel className="model-cancel-icon" />
            </div>
            <div className="model-switch-wrapper">
                <div className={ `switch-indicator ${typeProp}` } />
                <button onClick={ () => setTypeProp("nodeType") } className="model-switch-btn">
                    Вершина
                </button>
                <button onClick={ () => setTypeProp("edgeType") } className="model-switch-btn">
                    Переходы
                </button>
            </div>
            <div className="prop-type-wrapper">
                {typeProp === "nodeType" ? (
                    <ModelPropertiesNodeWidget />
                ) :
                    (
                        <ModelPropertiesEdgeWidget />
                    )}
            </div>
        </div>
    );
}