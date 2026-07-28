import History from "@/assets/svg/History.svg?react";
import ArrowRight from "@/assets/svg/ArrowRight.svg?react";
import CircleDot from "@/assets/svg/CircleDot.svg?react";
import ArrowUp from "@/assets/svg/ArrowUp.svg?react";
import Processor from "@/assets/svg/Processor.svg?react";
import styles from "../../scenes/MainScene.module.scss";

export default function MainHistoryWidget () {
    return (
        <>
            <div className={ styles.recentCleanWrapper }>
                <p className={ styles.recentTitle }>
                    <History />
                    Недавние
                </p>
                <p className={ styles.cleanText }>Очистить список</p>
            </div>
            <div className={ styles.recentCardsWrapper }>
                <div className={ styles.recentCard }>
                    <CircleDot className={ styles.recentIcon } />
                    <div className={ styles.recentNameTimeCardWrapper }>
                        <p className={ styles.recentName }>DFA_Binary.jff</p>
                        <p className={ styles.recentTime }>2 часа назад</p>
                    </div>
                    <ArrowRight className={ styles.recentArrow } />
                </div>
                <div className={ styles.recentCard }>
                    <Processor className={ styles.recentIcon } />
                    <div className={ styles.recentNameTimeCardWrapper }>
                        <p className={ styles.recentName }>lab_4.jff</p>
                        <p className={ styles.recentTime }>Вчера, 19:43</p>
                    </div>
                    <ArrowRight className={ styles.recentArrow } />
                </div>
                <div className={ styles.recentCard }>
                    <ArrowUp className={ styles.recentIcon } />
                    <div className={ styles.recentNameTimeCardWrapper }>
                        <p className={ styles.recentName }>kc_grammar.jff</p>
                        <p className={ styles.recentTime }>3 дня назад</p>
                    </div>
                    <ArrowRight className={ styles.recentArrow } />
                </div>
                <div className={ styles.recentCard }>
                    <Processor className={ styles.recentIcon } />
                    <div className={ styles.recentNameTimeCardWrapper }>
                        <p className={ styles.recentName }>lab_2.jff</p>
                        <p className={ styles.recentTime }>13.02.2026</p>
                    </div>
                    <ArrowRight className={ styles.recentArrow } />
                </div>
            </div>
        </>
    );
}