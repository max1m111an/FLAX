import { createHashRouter } from "react-router-dom";
import { ROUTES } from "./RoutesConst";

import MainLayout from "@/layouts/MainLayout";
import MainScene from "@/scenes/MainScene.tsx";
import SettingsScene from "@/scenes/SettingsScene.tsx";
import ModelScene from "@/scenes/ModelScene.tsx";
import { DebugScene } from "@/scenes/DebugScene.tsx";

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
            {
                path: ROUTES.DEBUG,
                element: <DebugScene />,
            },
        ],
    },
];

export const router = createHashRouter(routesConfig);