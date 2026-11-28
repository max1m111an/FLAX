import { invoke } from "@tauri-apps/api/core";
import { LoadJFFRequest, LoadJFFResponse, SaveJFFRequest, SaveJFFResponse } from "@/types/api/jff.ts";


export const jffAPI = {
    async loadJFF (params: LoadJFFRequest): Promise<LoadJFFResponse> {
        try {
            const response = await invoke<LoadJFFResponse>("load_file", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_load:", error);
            throw error;
        }
    },
    async saveJFF  (params: SaveJFFRequest): Promise<SaveJFFResponse> {
        try {
            const response = await invoke<SaveJFFResponse>("save_file", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_save:", error);
            throw error;
        }
    },
};

