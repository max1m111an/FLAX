import { createContext, useContext, ReactNode, useState } from "react";

interface RecentFilesContextProps {
    files: RecentFile[];
    addFile: (path: string) => void;
    clearFiles: () => void;
    removeFile: (path: string) => void;
}

export interface RecentFile {
    path: string;
    name: string;
    timestamp: number;
}

const STORAGE_KEY = "recent-files";
const MAX_FILES = 10;

const RecentFilesContext = createContext<RecentFilesContextProps | undefined>(undefined);

export const RecentFilesProvider = ({ children }: { children: ReactNode }) => {
    const [ files, setFiles ] = useState<RecentFile[]>(() => {
        try {
            return JSON.parse(localStorage.getItem(STORAGE_KEY) || "[]");
        } catch {
            return [];
        }
    });

    const persist = (next: RecentFile[]) => {
        setFiles(next);
        localStorage.setItem(STORAGE_KEY, JSON.stringify(next));
    };

    const addFile = (path: string) => {
        const name = path.split(/[/\\]/).filter(Boolean).pop() || path;
        const updated = [
            { path, name, timestamp: Date.now() },
            ...files.filter((f) => f.path !== path),
        ].slice(0, MAX_FILES);
        persist(updated);
    };

    const clearFiles = () => {
        persist([]);
    };

    const removeFile = (path: string) => {
        persist(files.filter((f) => f.path !== path));
    };

    return (
        <RecentFilesContext.Provider value={ { files, addFile, clearFiles, removeFile } }>
            {children}
        </RecentFilesContext.Provider>
    );
};
// eslint-disable-next-line react-refresh/only-export-components
export const useRecentFiles = () => {
    const context = useContext(RecentFilesContext);
    if (!context) throw new Error("useRecentFiles must be used within RecentFilesProvider");
    return context;
};