import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider } from "react-router-dom";
import { router } from "@/configs/RoutesConfig.tsx";

import { ThemeProvider } from "@/context/ThemeContext.tsx";

const root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(
    <React.StrictMode>
        <ThemeProvider>
            <RouterProvider router={ router } />
        </ThemeProvider>
    </React.StrictMode>,
);
