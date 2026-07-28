import Play from "@/assets/svg/Play.svg?react";
import FileDown from "@/assets/svg/FileDown.svg?react";
import { Button } from "@/components/ui/Button/Button.tsx";
import { TextArea } from "@/components/ui/Textfield/Textfield.tsx";
import { Typography } from "@/components/ui/Typography/Typography.tsx";
import styles from "./ModelTestWidget.module.scss";

export default function MultiTesting() {
    return (
        <>
            <Typography variant="pretitle">Входные строки</Typography>
            <TextArea
                rows={ 5 }
            />
            <div className={ styles.playExportWrapper }>
                <Button variant="main">
                    <Play />
                    Запустить все
                </Button>
                <button className={ styles.exportMultiTestBtn }>
                    <FileDown />
                    Экспорт
                </button>
            </div>
        </>
    );
}