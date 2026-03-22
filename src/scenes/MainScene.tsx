import "@/assets/scss/index.scss";
import MainModelWidget from "@/widgets/MainModelWidget.tsx";
import MainControlWidget from "@/widgets/MainControlWidget.tsx";
import MainHistoryWidget from "@/widgets/MainHistoryWidget.tsx";


export default function MainScene() {
    return (
        <div className="main-container">
            <p className="main-title">Новый проект</p>
            <p className="main-subtitle">Создайте инструмент для работы с с формальными языками или загрузите существующий файл</p>
            <MainModelWidget />
            <MainControlWidget />
            <MainHistoryWidget />
            <div className="bottom-space" />
        </div>
    );
}