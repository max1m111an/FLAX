import Cancel from "@/assets/svg/Cancel.svg?react";
import { useState } from "react";
import SoloTesting from "./SoloTesting.tsx";
import FileTesting from "./FileTesting.tsx";
import MultiTesting from "./MultiTesting.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import { IconButton } from "@/components/ui/IconButton/IconButton.tsx";
import clsx from "clsx";
import styles from "./ModelTestWidget.module.scss";

export default function ModelTestWidget() {
    const [ typeTest, setTypeTest ] = useState<string>("solo");
    return (
        <div className={ styles.wrapper }>
            <div className={ styles.titleCancelWrapper }>
                <Typography variant="title">Тестирование</Typography>
                <IconButton variant="cancel" onClick={ () => {} }>
                    <Cancel />
                </IconButton>
            </div>
            <div className={ styles.switchWrapper }>
                <div className={ clsx(styles.switchIndicator, styles[typeTest]) } />
                <button
                    onClick={ () => setTypeTest("solo") }
                    className={ clsx(styles.switchBtn, typeTest === "solo" && styles.active) }
                >
                    Единичный
                </button>
                <button
                    onClick={ () => setTypeTest("multi") }
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
                    <>
                        <MultiTesting />
                        <FileTesting />
                    </>
                )}
            </div>
        </div>
    );
}