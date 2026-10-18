import {
    createContext,
    HTMLAttributes,
    ReactNode,
    useContext,
    useEffect,
    useLayoutEffect,
    useRef,
    useState,
} from "react";
import { createPortal } from "react-dom";
import styles from "./ContextMenu.module.scss";


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
    children: ReactNode;
}
const ContextMenuContext = createContext<ContextMenuContextType | null>(null);

export const ContextMenuItem = ({ children, ...props }: MenuItemProps) => {
    return (
        <div className={ styles.item } { ...props }>
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

        document.addEventListener("click", closeMenu);
        window.addEventListener("scroll", handleScroll, { passive: true });
        window.addEventListener("resize", closeMenu);

        return () => {
            document.removeEventListener("click", closeMenu);
            window.removeEventListener("scroll", handleScroll);
            window.removeEventListener("resize", closeMenu);
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