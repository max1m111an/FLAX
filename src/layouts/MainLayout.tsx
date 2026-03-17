import { Outlet } from "react-router-dom";
import "@/assets/scss/index.scss";
import TabsWidget from "@/widgets/TabsWidget.tsx";


export default function MainLayout() {
    return (
        <div className="app-container">
            <TabsWidget />
            <main className="main-content">
                <Outlet />
            </main>
        </div>
    );
}
