import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "node:path";
import svgr from 'vite-plugin-svgr';

const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
    plugins: [react(),
        svgr({
            svgrOptions: {
                icon: true,
            },
        }),],

    resolve: {
        alias: {
            "@": path.resolve(__dirname, "src"),
        },
    },
    css: {
        preprocessorOptions: {
            scss: {
                additionalData: "@use \"@/assets/scss/consts\" as *;",
            },
        },
    },

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: "ws",
                host,
                port: 1421,
            }
            : undefined,
        watch: {
            ignored: ["**/src-tauri/**"],
        },
    },
}));