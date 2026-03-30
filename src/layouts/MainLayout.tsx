import { Outlet } from "react-router-dom";
import "@/assets/scss/index.scss";
import TabsWidget from "@/widgets/TabsWidget.tsx";
import { TabsProvider } from "@/context/TabsContext.tsx";
import { RightControlProvider } from "@/context/RightControlContext.tsx";


export default function MainLayout() {
    return (
        <RightControlProvider>
            <TabsProvider>
                <div className="app-container">
                    <TabsWidget />
                    <main className="container">
                        <Outlet />
                    </main>
                </div>
            </TabsProvider>
        </RightControlProvider>
    );
}
