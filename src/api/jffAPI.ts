import { invoke } from "@tauri-apps/api/core";
import { AutomatonModel } from "@/types/Automaton.ts";

type loadJFFRequest = {
    path: string;
}

type loadJFFResponse = {
    status: number;
    message: string;
    automaton?: AutomatonModel;
}

export const loadJFF = async (params: loadJFFRequest): Promise<loadJFFResponse> => {
    try {
        const response = await invoke<loadJFFResponse>("load_jff", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове save_jff:", error);
        throw error;
    }
};
