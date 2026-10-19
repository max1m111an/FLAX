import { Outlet } from "react-router-dom";
import "@/assets/scss/index.scss";
import TabsWidget from "@/widgets/Navigation/TabsWidget.tsx";
import { TabsProvider } from "@/context/TabsContext.tsx";
import { RecentFilesProvider } from "@/context/RecentFilesContext.tsx";
import styles from "./MainLayout.module.scss";
import { SnackbarProvider } from "@/context/SnackbarContext.tsx";
import { ContextMenuContextProvider } from "@/context/ContextMenuContext.tsx";


export default function MainLayout() {
    return (
        <ContextMenuContextProvider>
            <SnackbarProvider>
                <RecentFilesProvider>
                    <TabsProvider>
                        <div className={ styles.appContainer }>
                            <TabsWidget />
                            <main className={ styles.container }>
                                <Outlet />
                            </main>
                        </div>
                    </TabsProvider>
                </RecentFilesProvider>
            </SnackbarProvider>
        </ContextMenuContextProvider>
    );
}
