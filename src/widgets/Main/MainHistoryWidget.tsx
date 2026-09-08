import { useEffect, useState } from "react";
import History from "@/assets/svg/History.svg?react";
import ArrowRight from "@/assets/svg/ArrowRight.svg?react";
import CircleDot from "@/assets/svg/CircleDot.svg?react";
import { useRecentFiles, RecentFile } from "@/context/RecentFilesContext.tsx";
import { useTabs } from "@/context/TabsContext.tsx";
import { models } from "@/data/models.ts";
import { loadJff } from "@/services/jffService.ts";
import styles from "../../scenes/MainScene.module.scss";

const pluralHours = (n: number): string => {
    const mod10 = n % 10;
    const mod100 = n % 100;
    if (mod10 === 1 && mod100 !== 11) return "час";
    if (mod10 >= 2 && mod10 <= 4 && (mod100 < 12 || mod100 > 14)) return "часа";
    return "часов";
};

const pluralDays = (n: number): string => {
    const mod10 = n % 10;
    const mod100 = n % 100;
    if (mod10 === 1 && mod100 !== 11) return "день";
    if (mod10 >= 2 && mod10 <= 4 && (mod100 < 12 || mod100 > 14)) return "дня";
    return "дней";
};

const pad = (n: number): string => String(n).padStart(2, "0");

const formatTime = (ts: number, now: number): string => {
    const nowDate = new Date(now);
    const date = new Date(ts);
    const startOfToday = new Date(nowDate.getFullYear(), nowDate.getMonth(), nowDate.getDate()).getTime();
    const elapsed = now - ts;

    if (ts >= startOfToday) {
        const minutes = Math.floor(elapsed / 60000);
        if (minutes < 1) return "Только что";
        if (minutes < 60) return `${minutes} мин. назад`;
        return `${Math.floor(minutes / 60)} ${pluralHours(Math.floor(minutes / 60))} назад`;
    }

    const time = `${pad(date.getHours())}:${pad(date.getMinutes())}`;
    if (ts >= startOfToday - 86400000) {
        return `Вчера, ${time}`;
    }
    if (ts >= startOfToday - 2 * 86400000) {
        return `Позавчера, ${time}`;
    }

    const days = Math.floor((startOfToday - ts) / 86400000);
    if (days < 7) {
        return `${days} ${pluralDays(days)} назад`;
    }
    return `${pad(date.getDate())}.${pad(date.getMonth() + 1)}.${date.getFullYear()}`;
};

const pathSegments = (path: string): string[] =>
    path.split(/[/\\]/).filter((seg) => seg.length > 0);

const suffixAt = (path: string, depth: number): string => {
    const segs = pathSegments(path);
    const take = Math.min(depth - 1, segs.length - 1);
    return segs.slice(segs.length - 1 - take).join("/");
};

const getDisplayNames = (files: RecentFile[]): Map<string, string> => {
    const pathsByName = new Map<string, string[]>();
    for (const file of files) {
        const list = pathsByName.get(file.name) ?? [];
        list.push(file.path);
        pathsByName.set(file.name, list);
    }

    const display = new Map<string, string>();
    for (const [ name, paths ] of pathsByName) {
        if (paths.length === 1) {
            display.set(paths[0], name);
            continue;
        }
        let depth = 1;
        while (true) {
            let unique = true;
            const seen = new Set<string>();
            for (const path of paths) {
                const suffix = suffixAt(path, depth);
                if (seen.has(suffix)) {
                    unique = false;
                    break;
                }
                seen.add(suffix);
            }
            for (const path of paths) {
                display.set(path, suffixAt(path, depth));
            }
            if (unique) break;
            depth++;
        }
    }
    return display;
};

export default function MainHistoryWidget () {
    const { files, addFile, clearFiles } = useRecentFiles();
    const { loadTab } = useTabs();
    const [ now, setNow ] = useState(() => Date.now());

    useEffect(() => {
        const timer = setInterval(() => setNow(Date.now()), 60_000);
        return () => clearInterval(timer);
    }, []);

    if (files.length === 0) {
        return null;
    }

    const displayNames = getDisplayNames(files);

    const openRecent = async (path: string) => {
        const response = await loadJff({ path });
        if (response.automaton) {
            loadTab(response.automaton, models[0], path);
            addFile(path);
        }
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
                {files.map((file) => (
                    <div key={ file.path } className={ styles.recentCard } onClick={ () => openRecent(file.path) }>
                        <CircleDot className={ styles.recentIcon } />
                        <div className={ styles.recentNameTimeCardWrapper }>
                            <p className={ styles.recentName }>{displayNames.get(file.path) ?? file.name}</p>
                            <p className={ styles.recentTime }>{formatTime(file.timestamp, now)}</p>
                        </div>
                        <ArrowRight className={ styles.recentArrow } />
                    </div>
                ))}
            </div>
        </>
    );
}