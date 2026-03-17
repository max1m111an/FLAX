import "@/assets/scss/index.scss";
import CircleDot from "@/assets/svg/CircleDot.svg?react";
import ArrowUp from "@/assets/svg/ArrowUp.svg?react";
import Processor from "@/assets/svg/Processor.svg?react";
import FolderOpen from "@/assets/svg/FolderOpen.svg?react";
import Settings from "@/assets/svg/Settings.svg?react";
import Documentation from "@/assets/svg/Documentation.svg?react";
import History from "@/assets/svg/History.svg?react";
import ArrowRight from "@/assets/svg/ArrowRight.svg?react";


export default function MainScene() {
    return (
        <div className="main-container">
            <p className="main-title">Новый проект</p>
            <p className="main-subtitle">Создайте инструмент для работы с с формальными языками или загрузите существующий файл</p>
            <div className="cards-wrapper">
                <div className="card-type">
                    <CircleDot className="icon-container" />

                    <p className="card-title-type">Конечный автомат</p>
                    <p className="card-description-type">Моделирование НКА, ДКА</p>
                </div>
                <div className="card-type">
                    <ArrowUp className="icon-container" />
                    <p className="card-title-type">КС-грамматика</p>
                    <p className="card-description-type">Работа с магазинными автоматами для КС-языков</p>
                </div>
                <div className="card-type">
                    <Processor className="icon-container" />
                    <p className="card-title-type">Машина Тьюринга</p>
                    <p className="card-description-type">Создание полных вычислительных моделей</p>
                </div>
            </div>
            <div className="control-wrapper">
                <button className="open-file-button">
                    <FolderOpen />
                    Открыть файл (.jff)
                </button>
                <button className="control-button">
                    <Documentation />
                    Документация
                </button>
                <button className="control-button">
                    <Settings />
                    Настройки
                </button>
            </div>
            <div className="recent-clean-wrapper">
                <p className="recent-title">
                    <History />
                    Недавние
                </p>
                <p className="clean-text">Очистить список</p>
            </div>
            <div className="recent-cards-wrapper">
                <div className="recent-card">
                    <CircleDot className="recent-icon" />
                    <div className="name-time-card-wrapper">
                        <p className="recent-name">DFA_Binary.jff</p>
                        <p className="recent-time">2 часа назад</p>
                    </div>
                    <ArrowRight className="recent-arrow" />
                </div>
                <div className="recent-card">
                    <Processor className="recent-icon" />
                    <div className="name-time-card-wrapper">
                        <p className="recent-name">lab_4.jff</p>
                        <p className="recent-time">Вчера, 19:43</p>
                    </div>
                    <ArrowRight className="recent-arrow" />
                </div>
                <div className="recent-card">
                    <ArrowUp className="recent-icon" />
                    <div className="name-time-card-wrapper">
                        <p className="recent-name">kc_grammar.jff</p>
                        <p className="recent-time">3 дня назад</p>
                    </div>
                    <ArrowRight className="recent-arrow" />
                </div>
                <div className="recent-card">
                    <Processor className="recent-icon" />
                    <div className="name-time-card-wrapper">
                        <p className="recent-name">lab_2.jff</p>
                        <p className="recent-time">13.02.2026</p>
                    </div>
                    <ArrowRight className="recent-arrow" />
                </div>
            </div>
        </div>
    );
}