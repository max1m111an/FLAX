import Cancel from "@/assets/svg/Cancel.svg?react";
import { useState } from "react";
import NodeProperties from "./NodeProperties.tsx";
import EdgeProperties from "./EdgeProperties.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import { IconButton } from "@/components/ui/IconButton/IconButton.tsx";
import clsx from "clsx";
import styles from "./ModelProperties.module.scss";
import { useCurrentTab, useTabs } from "@/context/TabsContext.tsx";

export default function ModelPropertiesWidget() {
    const [ typeProp, setTypeProp ] = useState<string>("nodeType");
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();

    return (
        <div className={ styles.wrapper }>
            <div className={ styles.titleCancelWrapper }>
                <Typography variant="title">Свойства</Typography>
                <IconButton variant="cancel" onClick={ () => {
                    if (currentTab) {
                        updateTab({
                            ...currentTab,
                            activePane: null,
                        });
                    }
                } }>
                    <Cancel />
                </IconButton>
            </div>
            <div className={ styles.switchWrapper }>
                <div className={ clsx(styles.switchIndicator, styles[typeProp]) } />
                <button
                    onClick={ () => setTypeProp("nodeType") }
                    className={ clsx(styles.switchBtn, typeProp === "nodeType" && styles.active) }
                >
                    Состояние
                </button>
                <button
                    onClick={ () => setTypeProp("edgeType") }
                    className={ clsx(styles.switchBtn, typeProp === "edgeType" && styles.active) }
                >
                    Переходы
                </button>
            </div>
            <div className={ styles.propTypeWrapper }>
                {typeProp === "nodeType" ? (
                    <NodeProperties />
                ) :
                    (
                        <EdgeProperties />
                    )}
            </div>
        </div>
    );
}