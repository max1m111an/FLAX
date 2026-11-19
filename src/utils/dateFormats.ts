import { RecentFile } from "@/context/RecentFilesContext.tsx";

/**Преобразование в формат: час, часа, часов*/
const pluralHours = (n: number): string => {
    const mod10 = n % 10;
    const mod100 = n % 100;
    if (mod10 === 1 && mod100 !== 11) return "час";
    if (mod10 >= 2 && mod10 <= 4 && (mod100 < 12 || mod100 > 14)) return "часа";
    return "часов";
};

/**Преобразование в формат: днеь, дня, дней*/
const pluralDays = (n: number): string => {
    const mod10 = n % 10;
    const mod100 = n % 100;
    if (mod10 === 1 && mod100 !== 11) return "день";
    if (mod10 >= 2 && mod10 <= 4 && (mod100 < 12 || mod100 > 14)) return "дня";
    return "дней";
};

/**Добавляет ведущий ноль в начало строки*/
const pad = (n: number): string => String(n).padStart(2, "0");

/**Преобразование в формат: сегодня, вчера, позавчера, N дней назад, хх.хх.хххх*/
export const formatTime = (ts: number, now: number): string => {
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

/**Разбивает путь к файлу на непустые сегменты (поддерживает `/` и `\`)*/
const pathSegments = (path: string): string[] =>
    path.split(/[/\\]/).filter((seg) => seg.length > 0);

const suffixAt = (path: string, depth: number): string => {
    /**Возвращает хвост пути из последних `depth` сегментов, разделённых `/`*/
    const segs = pathSegments(path);
    const take = Math.min(depth - 1, segs.length - 1);
    return segs.slice(segs.length - 1 - take).join("/");
};

/**
 Формирует карту `путь → отображаемое имя` для недавних файлов.
 Если имена файлов совпадают, добавляет родительские папки до достижения уникальности.
 */
export const getDisplayNames = (files: RecentFile[]): Map<string, string> => {
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