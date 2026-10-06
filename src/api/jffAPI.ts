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
        const response = await invoke<loadJFFResponse>("fa_load", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове fa_load:", error);
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
        const response = await invoke<saveJFFResponse>("fa_save", params);
        return response;
    } catch (error) {
        console.error("Ошибка при вызове fa_save:", error);
        throw error;
    }
};

