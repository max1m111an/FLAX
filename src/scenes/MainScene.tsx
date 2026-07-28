import MainModelWidget from "@/widgets/Main/MainModelWidget.tsx";
import MainControlWidget from "@/widgets/Main/MainControlWidget.tsx";
import MainHistoryWidget from "@/widgets/Main/MainHistoryWidget.tsx";
import styles from "./MainScene.module.scss";


export default function MainScene() {
    return (
        <div className={ styles.mainContainer }>
            <p className={ styles.mainTitle }>Новый проект</p>
            <p className={ styles.mainSubtitle }>Создайте инструмент для работы с с формальными языками или загрузите существующий файл</p>
            <MainModelWidget />
            <MainControlWidget />
            <MainHistoryWidget />
            <div className={ styles.bottomSpace } />
        </div>
    );
}