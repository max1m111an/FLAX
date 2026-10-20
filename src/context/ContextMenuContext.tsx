import {
    createContext,
    HTMLAttributes, InputHTMLAttributes,
    ReactNode,
    useContext,
    useEffect,
    useLayoutEffect,
    useRef,
    useState,
} from "react";
import { createPortal } from "react-dom";
import styles from "./ContextMenu.module.scss";
import clsx from "clsx";


interface MenuState {
    visible: boolean;
    x: number;
    y: number;
    content: ReactNode | null;
}

interface ContextMenuContextType {
    setMenuState: React.Dispatch<React.SetStateAction<MenuState>>;
}

interface ProviderProps {
    children: ReactNode;
}

interface MenuItemProps extends HTMLAttributes<HTMLDivElement> {
    isRed?: boolean;
    isInput?: boolean;
    children: ReactNode;
}
const ContextMenuContext = createContext<ContextMenuContextType | null>(null);

type MenuInputProps = InputHTMLAttributes<HTMLInputElement>;

export const ContextMenuInput = (props: MenuInputProps) => {
    return <input className={ styles.input } { ...props } />;
};

export const ContextMenuItem = ({ isRed = false, isInput = false, children, ...props }: MenuItemProps) => {
    return (
        <div className={ clsx(styles.item, isRed && styles.red, isInput && styles.inputContainer) } { ...props }>
            {children}
        </div>
    );
};

export const ContextMenuContextProvider = ({ children }: ProviderProps) => {
    const [ menuState, setMenuState ] = useState<MenuState>({
        visible: false,
        x: 0,
        y: 0,
        content: null,
    });
    const menuRef = useRef<HTMLDivElement>(null);

    const closeMenu = () => setMenuState((prev) => ({ ...prev, visible: false }));

    useEffect(() => {
        const handleScroll = () => closeMenu();
        const handleKeyDown = (e: KeyboardEvent) => {
            if (e.key === "Escape") {
                console.log("Клавиша:", e.key);
                closeMenu();
            }
        };

        document.addEventListener("click", closeMenu);
        window.addEventListener("scroll", handleScroll, { passive: true });
        window.addEventListener("resize", closeMenu);
        window.addEventListener("keydown", handleKeyDown, true);
        return () => {
            document.removeEventListener("click", closeMenu);
            window.removeEventListener("scroll", handleScroll);
            window.removeEventListener("resize", closeMenu);
            window.removeEventListener("keydown", handleKeyDown, true);
        };
    }, []);

    useLayoutEffect(() => {
        if (!menuState.visible || !menuRef.current) return;

        const menu = menuRef.current;

        const rect = menu.getBoundingClientRect();

        let adjustedX = menuState.x;
        let adjustedY = menuState.y;

        const windowWidth = window.innerWidth;
        const windowHeight = window.innerHeight;

        if (menuState.x + rect.width > windowWidth) {
            adjustedX = menuState.x - rect.width;
        }

        if (menuState.y + rect.height > windowHeight) {
            adjustedY = menuState.y - rect.height;
        }

        adjustedX = Math.max(10, adjustedX);
        adjustedY = Math.max(10, adjustedY);

        menu.style.left = `${adjustedX}px`;
        menu.style.top = `${adjustedY}px`;

        menu.style.opacity = "1";

    }, [ menuState ]);

    return (
        <ContextMenuContext.Provider value={ { setMenuState } }>
            {children}
            {menuState.visible && createPortal(
                <div
                    className={ styles.globalContextMenu }
                    style={ { top: menuState.y, left: menuState.x } }
                    ref={ menuRef }
                    onContextMenu={ (e) => e.preventDefault() }
                >
                    {menuState.content}
                </div>,
                document.body,
            )}
        </ContextMenuContext.Provider>);
};
// eslint-disable-next-line react-refresh/only-export-components
export const useContextMenu = () => {
    const context = useContext(ContextMenuContext);
    if (!context) {
        throw new Error("useContextMenu должен использоваться внутри ContextMenuProvider");
    }
    return context;
};