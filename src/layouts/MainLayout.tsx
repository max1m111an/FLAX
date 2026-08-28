import { Outlet } from "react-router-dom";
import "@/assets/scss/index.scss";
import TabsWidget from "@/widgets/Navigation/TabsWidget.tsx";
import { TabsProvider } from "@/context/TabsContext.tsx";
import styles from "./MainLayout.module.scss";

export default function MainLayout() {
    return (
        <TabsProvider>
            <div className={ styles.appContainer }>
                <TabsWidget />
                <main className={ styles.container }>
                    <Outlet />
                </main>
            </div>
        </TabsProvider>
    );
}
