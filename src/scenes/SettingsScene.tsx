import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { useTheme } from "@/context/ThemeContext.tsx";

export default function SettingsScene() {
    const { theme, setTheme } = useTheme();
    return (
        <>
            <DropdownMenu.Root>
                <DropdownMenu.Trigger className="plus-button">
                    {theme}
                </DropdownMenu.Trigger>

                <DropdownMenu.Content className="dropdown">
                    <DropdownMenu.Group>
                        <>
                            <DropdownMenu.Item className="item" onSelect={ () => setTheme("Светлая") }>
                                Светлая
                            </DropdownMenu.Item>
                            <DropdownMenu.Item className="item" onSelect={ () => setTheme("Темная") }>
                                Темная
                            </DropdownMenu.Item>
                        </>
                    </DropdownMenu.Group>
                </DropdownMenu.Content>
            </DropdownMenu.Root>
        </>
    );
}