import { createBrowserRouter } from "react-router-dom";
import { ROUTES } from "./RoutesConst";

import MainLayout from "@/layouts/MainLayout";
import MainScene from "@/scenes/MainScene.tsx";

export const routesConfig = [
    {
        element: <MainLayout />,
        children: [
            {
                path: ROUTES.MAIN,
                element: <MainScene />,
            },
            {
                path: ROUTES.SETTINGS,
                element: <MainScene />,
            },
        ],
    },
];

export const router = createBrowserRouter(routesConfig);
