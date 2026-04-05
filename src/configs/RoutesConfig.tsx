import { createHashRouter } from "react-router-dom";
import { ROUTES } from "./RoutesConst";

import MainLayout from "@/layouts/MainLayout";
import MainScene from "@/scenes/MainScene.tsx";
import SettingsScene from "@/scenes/SettingsScene.tsx";
import ModelScene from "@/scenes/ModelScene.tsx";

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
                element: <SettingsScene />,
            },
            {
                path: ROUTES.MODELS,
                element: <ModelScene />,
            },
        ],
    },
];

export const router = createHashRouter(routesConfig);