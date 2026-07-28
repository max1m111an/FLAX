import { models } from "@/data/models.ts";
import { useTabs } from "@/context/TabsContext.tsx";
import styles from "../../scenes/MainScene.module.scss";

export default function MainModelWidget() {
    const { addTab } = useTabs();

    return (
        <div className={ styles.cardsWrapper }>
            {models.map((model) => (
                <div key={ model.type } className={ styles.cardType } onClick={ (_e) => addTab(model) }>
                    <model.icon className={ styles.iconContainer } />

                    <p className={ styles.cardTitleType }>{model.type}</p>
                    <p className={ styles.cardDescriptionType }>{model.description}</p>
                </div>
            ))}
        </div>
    );
}