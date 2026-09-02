import Cancel from "@/assets/svg/Cancel.svg?react";
import SoloTesting from "./SoloTesting.tsx";
import MultiTesting from "./MultiTesting.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import { IconButton } from "@/components/ui/IconButton/IconButton.tsx";
import clsx from "clsx";
import styles from "./ModelTestWidget.module.scss";
import { useCurrentTab, useTabs } from "@/context/TabsContext.tsx";

export default function ModelTestWidget() {
    const currentTab = useCurrentTab();
    const { updateTab } = useTabs();

    const typeTest = currentTab?.testMode ?? "solo";

    const switchType = (type: string) => {
        if (currentTab) {
            updateTab({ ...currentTab, testMode: type });
        }
    };

    const closePanel = () => {
        if (currentTab) {
            updateTab({
                ...currentTab,
                activePanel: null,
                selectedState: null,
                selectedTransition: null,
                selectedNodeId: null,
                pendingTestLine: null,
                pendingTraces: null,
            });
        }
    };

    return (
        <div className={ styles.wrapper }>
            <div className={ styles.titleCancelWrapper }>
                <Typography variant="title">Тестирование</Typography>
                <IconButton variant="cancel" onClick={ closePanel }>
                    <Cancel />
                </IconButton>
            </div>
            <div className={ styles.switchWrapper }>
                <div className={ clsx(styles.switchIndicator, styles[typeTest]) } />
                <button
                    onClick={ () => switchType("solo") }
                    className={ clsx(styles.switchBtn, typeTest === "solo" && styles.active) }
                >
                    Единичный
                </button>
                <button
                    onClick={ () => switchType("multi") }
                    className={ clsx(styles.switchBtn, typeTest === "multi" && styles.active) }
                >
                    Мульти
                </button>
            </div>
            <div className={ styles.testTypeWrapper }>
                {typeTest === "solo" && (
                    <SoloTesting />
                )}
                {typeTest === "multi" && (
                    <MultiTesting />
                )}
            </div>
        </div>
    );
}