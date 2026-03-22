import History from "@/assets/svg/History.svg?react";
import ArrowRight from "@/assets/svg/ArrowRight.svg?react";
import CircleDot from "@/assets/svg/CircleDot.svg?react";
import ArrowUp from "@/assets/svg/ArrowUp.svg?react";
import Processor from "@/assets/svg/Processor.svg?react";

export default function MainHistoryWidget () {
    return (
        <>
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
                    <div className="recent-name-time-card-wrapper">
                        <p className="recent-name">DFA_Binary.jff</p>
                        <p className="recent-time">2 часа назад</p>
                    </div>
                    <ArrowRight className="recent-arrow" />
                </div>
                <div className="recent-card">
                    <Processor className="recent-icon" />
                    <div className="recent-name-time-card-wrapper">
                        <p className="recent-name">lab_4.jff</p>
                        <p className="recent-time">Вчера, 19:43</p>
                    </div>
                    <ArrowRight className="recent-arrow" />
                </div>
                <div className="recent-card">
                    <ArrowUp className="recent-icon" />
                    <div className="recent-name-time-card-wrapper">
                        <p className="recent-name">kc_grammar.jff</p>
                        <p className="recent-time">3 дня назад</p>
                    </div>
                    <ArrowRight className="recent-arrow" />
                </div>
                <div className="recent-card">
                    <Processor className="recent-icon" />
                    <div className="recent-name-time-card-wrapper">
                        <p className="recent-name">lab_2.jff</p>
                        <p className="recent-time">13.02.2026</p>
                    </div>
                    <ArrowRight className="recent-arrow" />
                </div>
            </div>
        </>
    );
}