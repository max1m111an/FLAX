import Uploud from "@/assets/svg/Uploud.svg?react";
import styles from "./ModelTestWidget.module.scss";

export default function FileTesting() {
    return (
        <div className={ styles.importWrapper }>
            <Uploud className={ styles.importIcon } />
            <p className={ styles.importTitle }>Выберите файл</p>
            <p className={ styles.importPretitle }>.txt, .cvs</p>
            <button className={ styles.importBtn }>Обзор</button>
        </div>
    );
}