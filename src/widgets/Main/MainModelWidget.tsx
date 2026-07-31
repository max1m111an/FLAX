import { models } from "@/data/models.ts";
import { useTabs } from "@/context/TabsContext.tsx";

export default function MainModelWidget() {
    const { addTab } = useTabs();

    return (
        <div className="cards-wrapper">
            {models.map((model) => (
                <div className="card-type" onClick={ (_e) => addTab(model) }>
                    <model.icon className="icon-container" />

                    <p className="card-title-type">{model.type}</p>
                    <p className="card-description-type">{model.description}</p>
                </div>
            ))}
        </div>
    );
}