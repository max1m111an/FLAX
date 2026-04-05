import { Outlet } from "react-router-dom";
import "@/assets/scss/index.scss";
import TabsWidget from "@/widgets/TabsWidget.tsx";
import { TabsProvider } from "@/context/TabsContext.tsx";


export default function MainLayout() {
    return (
        <TabsProvider>
            <div className="app-container">
                <TabsWidget />
                <main className="main-content">
                    <Outlet />
                </main>
            </div>
        </TabsProvider>
    );
}
