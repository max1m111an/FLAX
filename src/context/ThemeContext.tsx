import { createContext, useContext, useState, ReactNode, useEffect } from "react";

type Theme = "Темная" | "Светлая";

interface ThemeContextProps {
    theme: Theme;
    setTheme: (theme: Theme) => void;
}

const ThemeContext = createContext<ThemeContextProps | undefined>(undefined);

export const ThemeProvider = ({ children }: { children: ReactNode }) => {
    const [ theme, setThemeState ] = useState<Theme>(() => {
        const saved = localStorage.getItem("app-theme");
        return (saved as Theme) || "Темная";
    });

    useEffect(() => {
        if (theme === "Светлая") {
            document.documentElement.setAttribute("data-theme", "light");
        } else {
            document.documentElement.setAttribute("data-theme", "dark");
        }
        localStorage.setItem("app-theme", theme);
    }, [ theme ]);

    const setTheme = (newTheme: Theme) => {
        setThemeState(newTheme);
    };

    return (
        <ThemeContext.Provider value={ { theme, setTheme } }>
            {children}
        </ThemeContext.Provider>
    );
};

// eslint-disable-next-line react-refresh/only-export-components
export const useTheme = () => {
    const context = useContext(ThemeContext);
    if (!context) throw new Error("useTheme must be used within ThemeProvider");
    return context;
};
