import { invoke } from "@tauri-apps/api/core";
import { loadJFFRequest, loadJFFResponse, saveJFFRequest, saveJFFResponse } from "@/types/api/jff.ts";


export const jffAPI = {
    async loadJFF (params: loadJFFRequest): Promise<loadJFFResponse> {
        try {
            const response = await invoke<loadJFFResponse>("fa_load", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_load:", error);
            throw error;
        }
    },
    async saveJFF  (params: saveJFFRequest): Promise<saveJFFResponse> {
        try {
            const response = await invoke<saveJFFResponse>("fa_save", params);
            return response;
        } catch (error) {
            console.error("Ошибка при вызове fa_save:", error);
            throw error;
        }
    },
};

