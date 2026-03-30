import { Outlet } from "react-router-dom";
import "@/assets/scss/index.scss";
import TabsWidget from "@/widgets/TabsWidget.tsx";
import { TabsProvider } from "@/context/TabsContext.tsx";
import { ControlProvider } from "@/context/ControlContext.tsx";


export default function MainLayout() {
    return (
        <ControlProvider>
            <TabsProvider>
                <div className="app-container">
                    <TabsWidget />
                    <main className="container">
                        <Outlet />
                    </main>
                </div>
            </TabsProvider>
        </ControlProvider>
    );
}
