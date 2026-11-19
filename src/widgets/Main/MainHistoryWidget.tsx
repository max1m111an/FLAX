import { useEffect, useState } from "react";
import History from "@/assets/svg/History.svg?react";
import FileDown from "@/assets/svg/CircleAlert.svg?react";
import ArrowRight from "@/assets/svg/ArrowRight.svg?react";
import { useRecentFiles } from "@/context/RecentFilesContext.tsx";
import { useTabs } from "@/context/TabsContext.tsx";
import styles from "../../scenes/MainScene.module.scss";
import { useNavigate } from "react-router-dom";
import { models } from "@/data/Models.ts";
import { formatTime, getDisplayNames } from "@/utils/dateFormats.ts";

export default function MainHistoryWidget() {
    const { files, clearFiles } = useRecentFiles();
    const { loadTab, tabs } = useTabs();
    const [ now, setNow ] = useState(() => Date.now());
    const navigate = useNavigate();

    useEffect(() => {
        const timer = setInterval(() => setNow(Date.now()), 60_000);
        return () => clearInterval(timer);
    }, []);

    if (files.length === 0) {
        return null;
    }

    const displayNames = getDisplayNames(files);

    const openRecent = async (path: string) => {
        const existingTab = tabs.find((tab) => tab.savedPath === path);
        if (existingTab) {
            navigate(`/models/${existingTab.id}`);
            return;
        }
        loadTab(path);
    };

    return (
        <>
            <div className={ styles.recentCleanWrapper }>
                <p className={ styles.recentTitle }>
                    <History />
                    Недавние
                </p>
                <p className={ styles.cleanText } onClick={ clearFiles }>Очистить список</p>
            </div>
            <div className={ styles.recentCardsWrapper }>
                {files.map((file) => {
                    const model = models.find((m) => m.type === file.icon);
                    const Icon = model ? model.icon : FileDown;

                    return (
                        <div
                            key={ file.path }
                            className={ styles.recentCard }
                            onClick={ () => openRecent(file.path) }
                        >
                            <Icon className={ styles.recentIcon } />

                            <div className={ styles.recentNameTimeCardWrapper }>
                                <p className={ styles.recentName }>{displayNames.get(file.path) ?? file.name}</p>
                                <p className={ styles.recentTime }>{formatTime(file.timestamp, now)}</p>
                            </div>
                            <ArrowRight className={ styles.recentArrow } />
                        </div>
                    );
                })}
            </div>
        </>
    );
}