import { invoke } from "@tauri-apps/api/core";
import { AutomatonModel } from "@/types/Automaton.ts";

export type loadJFFRequest = {
    path: string;
}

export type loadJFFResponse = {
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

export type saveJFFRequest = {
    automatonId: number;
    path: string;
}

export type saveJFFResponse = {
    status: number;
    message: string;
}

export const saveJFF = async (params: saveJFFRequest): Promise<saveJFFResponse> => {
    try {
        const response = await invoke<saveJFFResponse>("save_jff", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове save_jff:", error);
        throw error;
    }
};

